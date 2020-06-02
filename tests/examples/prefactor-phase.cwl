# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

cwlVersion: v1.0
class: CommandLineTool

baseCommand: [python]

label: "Find global phase offset"

hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

inputs:
  losoto:
    type: File
    format: http://revoltek.github.io/losoto/cookbook.pdf

  refstation_id:
    type: int?
    default: 2

  source_id:
    type: int?
    default: 0

arguments:
  - prefix: -c
    valueFrom: |
      from find_cal_global_phaseoffset_losoto import main
      main(losotoname="$(inputs.losoto.path)",
           store_basename="cwl",
           refstationID=$(inputs.refstation_id),
           sourceID=$(inputs.source_id)
      )

outputs:
    freqs_for_phase_array:
      type: File
      format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
      label: "Frequencies"
      outputBinding:
        glob: "freqs_for_phase_array.npy"

    phase_array:
      type: File
      format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
      label: "Phase solutions"
      outputBinding:
        glob: "cwl_phase_array.npy"

    station_names:
      type: File
      format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
      label: "Station names"
      outputBinding:
        glob: "cwl_station_names.npy"

    phase_xx_yy_offset:
      type: File
      format: image/png
      outputBinding:
        glob: "phase_xx_yy_offset.png"

$namespaces:
  s: http://schema.org/
$schemas:
  - https://schema.org/version/latest/schema.rdf



s:license: "https://mit-license.org/"
s:author:
  s:person.url: "http://orcid.org/0000-0002-6136-3724"
