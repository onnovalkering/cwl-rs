# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

cwlVersion: v1.0
class: CommandLineTool
baseCommand: [NDPPP, msout=calibrated.MS]

label: "DPPP flag and average"

hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

inputs:
  msin:
    type: Directory
    inputBinding:
      prefix: "msin="
      separate: False

  msin.datacolumn:
    type: string
    default: DATA
    inputBinding:
      prefix: "msin.datacolumn="
      separate: False

  msout.writefullresflag:
    type:
      type: enum
      symbols: ["True", "False"]
    default: "False"
    inputBinding:
      prefix: "msout.writefullresflag="
      separate: False

  msout.overwrite:
    type:
      type: enum
      symbols: ["True", "False"]
    default: "True"
    inputBinding:
      prefix: "msout.overwrite="
      separate: False

  flag.type:
    type: string
    default: filter
    inputBinding:
      prefix: "flag.type="
      separate: False

  flag.baseline:
    type: string
    inputBinding:
      prefix: "flag.baseline="
      separate: False

  filter.type:
    type: string
    default: filter
    inputBinding:
      prefix: "filter.type="
      separate: False

  filter.baseline:
    type: string
    default: "CS*, RS*&&"
    inputBinding:
      prefix: "filter.baseline="
      separate: False

  filter.remove:
    type:
      type: enum
      symbols: ["True", "False"]
    default: "True"
    inputBinding:
      prefix: "filter.remove="
      separate: False

  avg.type:
    type: string
    default: average
    inputBinding:
      prefix: "avg.type="
      separate: False

  avg.timestep:
    type: int
    inputBinding:
      prefix: "avg.timestep="
      separate: False

  avg.freqstep:
    type: int
    inputBinding:
      prefix: "avg.freqstep="
      separate: False

  flagamp.type:
    type: string
    default: "preflagger"
    inputBinding:
      prefix: "flagamp.type="
      separate: False

  flagamp.amplmin:
    type: string
    default: "1e-30"
    inputBinding:
      prefix: "flagamp.amplmin="
      separate: False

  steps:
    type: string
    default: "[flag,filter,avg,flagamp]"
    inputBinding:
      prefix: "steps="
      separate: False

  baseline:
    type: string
    default: "[CS013HBA*]"
    inputBinding:
      prefix: "baseline="
      separate: False

  avg.timeresolution:
    type: int
    default: 4
    inputBinding:
      prefix: "average.timeresolution="
      separate: False

  avg.freqresolution:
    type: string
    default: "48.82kHz"
    inputBinding:
      prefix: "average.freqresolution="
      separate: False

outputs:
  msout:
    type: Directory
    outputBinding:
      glob: "calibrated.MS"


$namespaces:
  s: http://schema.org/
$schemas:
  - https://schema.org/version/latest/schema.rdf



s:license: "https://mit-license.org/"
s:author:
  s:person.url: "http://orcid.org/0000-0002-6136-3724"
