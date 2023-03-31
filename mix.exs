defmodule TapiRrule.MixProject do
  use Mix.Project
  @version "0.1.8"
  @source_url "https://github.com/kenzhengguan/rrule"

  def project do
    [
      app: :tapi_rrule,
      version: @version,
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      description: "tapi rrule implementation"
    ]
  end

  # When publishing a library to with precompiled NIFs to Hex,
  # is is mandatory to include a checksum file (along with other
  # necessary files in the library).
  #
  # Refer to the "The release flow"
  # in the "Precompilation guide" for more details:
  # https://hexdocs.pm/rustler_precompiled/precompilation_guide.html#the-release-flow
  defp package do
    [
      files: [
        "lib",
        "native",
        "checksum-*.exs",
        "mix.exs"
      ],
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => @source_url}
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:rustler, "~> 0.27.0"},
      {:rustler_precompiled, "~> 0.4"}
    ]
  end
end
