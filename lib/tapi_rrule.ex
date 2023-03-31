defmodule TapiRrule do
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :tapi_rrule,
    crate: "tapirrule",
    base_url: "https://github.com/kenzhengguan/rrule/releases/download/v#{version}",
    force_build: System.get_env("TAPI_RRULE_BUILD") in ["1", "true"],
    version: version

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.

  @doc """
  Example:
  TapiRrule.r_range("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3", "2012-02-01", "2012-04-01")
  """
  def r_range(_, _, _), do: error()
  def error(), do: :erlang.nif_error(:nif_not_loaded)
end
