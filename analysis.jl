using DataFrames, CSV, AlgebraOfGraphics, CairoMakie

set_aog_theme!()

root = "arxiv-pdfs"

data_file = "data.csv"

function read_yearmonth_data(yearmonth)
	path = "$root/$yearmonth/$data_file"
	CSV.read(path, DataFrame)
end

function load_data()
    yearmonths = readdir(root)

    df = vcat(map(read_yearmonth_data, yearmonths)...)

    Base.summarysize(df)

    df
end

# Length of the title
function title_length(df)
	df.title_length = map(length, coalesce.(df.title, ""))
	d = data(df)
	m = mapping(:title_length => "title length")
	v = histogram(; bins=200)
	fig = draw(d * m * v)
  save("plots/title_length.svg", fig)
end

# Length of the title
function title_length_no_zero(df)
	df.title_length = map(length, coalesce.(df.title, ""))
  df = filter(:title_length => l -> l > 0, df)
	d = data(df)
	m = mapping(:title_length => "title length")
	v = histogram(; bins=200)
  fig = draw(d * m * v)
  save("plots/title_length_no_zero.svg", fig)
end

function has_title(df)
	df.has_title = map(isempty, coalesce.(df.title, ""))
	grouped = combine(groupby(df, [:has_title, :yearmonth]), nrow)
	d = data(grouped)
  m = mapping(:yearmonth => nonnumeric => "year & month", :nrow => "count", color=:has_title => renamer(true => "has title", false => "has no title") => "", dodge=:has_title)
	v = visual(BarPlot)
	fig = draw(d * m * v)
  save("plots/has_title.svg", fig)
end

function has_title_alt(df)
	df.has_title = map(isempty, coalesce.(df.title, ""))
	grouped = combine(groupby(df, [:has_title, :yearmonth]), nrow)

	d = data(grouped)
	m = mapping(:yearmonth => nonnumeric, :nrow, color=:has_title => nonnumeric, linestyle=:has_title => nonnumeric)
	v = visual(Scatter) + visual(Lines)
	draw(d * m * v)
end

# Length of the authors
function author_length(df)
	df.author_length = map(length, coalesce.(df.author, ""))
	d = data(df)
	m = mapping(:author_length => "author length")
	v = histogram(; bins=200)
	fig = draw(d * m * v)
  save("plots/author_length.svg", fig)
end

# Length of the authors
function author_length_no_zero(df)
	df.author_length = map(length, coalesce.(df.author, ""))
  df = filter(:author_length => l -> l > 0, df)
	d = data(df)
	m = mapping(:author_length => "author length")
	v = histogram(; bins=200)
	fig = draw(d * m * v)
  save("plots/author_length_no_zero.svg", fig)
end

function has_author(df)
	df.has_author = map(isempty, coalesce.(df.author, ""))
	grouped = combine(groupby(df, [:has_author, :yearmonth]), nrow)
	d = data(grouped)
  m = mapping(:yearmonth => nonnumeric => "year & month", :nrow => "count", color=:has_author => renamer(true => "has author", false => "has no author") => "", dodge=:has_author)
	v = visual(BarPlot)
	fig = draw(d * m * v)
  save("plots/has_author.svg", fig)
end

function has_author_alt(df)
	df.has_author = map(isempty, coalesce.(df.author, ""))
	grouped = combine(groupby(df, [:has_author, :yearmonth]), nrow)
	d = data(grouped)
	m = mapping(:yearmonth => nonnumeric, :nrow, color=:has_author => nonnumeric, linestyle=:has_author => nonnumeric)
	v = visual(Scatter) + visual(Lines)
	draw(d * m * v)
end

# What is the common punctuation for authors
function nonalpha(s)
	filter(c -> ispunct(c), s)
end


function author_punc(df)
	author_punc_df = flatten(transform(df, :author => ByRow(x -> split(nonalpha(coalesce(x, "")), "")) => :author_punc), :author_punc)
	author_punc_df = combine(groupby(author_punc_df, [:author_punc, :yearmonth]), nrow)
	sort(author_punc_df, :author_punc)

	d = data(author_punc_df)
	m = mapping(:author_punc, :nrow, stack=:yearmonth => nonnumeric, color=:yearmonth => nonnumeric)
	v = visual(BarPlot)
	draw(d * m * v)
end

# What is the most common punctuation for keywords
function keyword_punc(df)
	keyword_punc_df = flatten(transform(df, :keywords => ByRow(x -> split(nonalpha(coalesce(x, "")), "")) => :keyword_punc), :keyword_punc)
	keyword_punc_df = combine(groupby(keyword_punc_df, :keyword_punc), nrow)
	sort(keyword_punc_df, :keyword_punc)
	d = data(keyword_punc_df)
	m = mapping(:keyword_punc, :nrow)
	v = visual(BarPlot)
	draw(d * m * v, axis=(yscale=log10,))
end

function most_popular(df, col, count)
    first(sort(combine(groupby(df, col), nrow), :nrow, rev=true), count)
end

function rows_with_length(df, col, len)
    filter(col => v -> v == len, df)
end

function main()
    df = load_data()

    has_title(df)
    title_length(df)
    title_length_no_zero(df)
    println("titles with length 1")
    println(most_popular(rows_with_length(df, :title_length, 1), :title, 10))
    println("titles with length 2")
    println(most_popular(rows_with_length(df, :title_length, 2), :title, 10))
    println("most popular title length")
    println(most_popular(df, :title_length, 10))
    println("most popular titles")
    println(most_popular(df, :title, 10))

    has_author(df)
    author_length(df)
    author_length_no_zero(df)
    println("authors with length 1")
    println(most_popular(rows_with_length(df, :author_length, 1), :author, 10))
    println("authors with length 2")
    println(most_popular(rows_with_length(df, :author_length, 2), :author, 10))
    println("most popular author length")
    println(most_popular(df, :author_length, 10))
    println("most popular authors")
    println(most_popular(df, :author, 10))
end
