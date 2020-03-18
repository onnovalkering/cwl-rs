# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

class: CommandLineTool
cwlVersion: v1.0
$namespaces:
  s: 'http://schema.org/'
hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

baseCommand: [python]
inputs:
  - id: prepcals
    type: 'Directory[]'
    inputBinding:
      position: 0
    doc: list of calibrated measurement sets
outputs:
  - id: mapfile_paths
    type: File
    outputBinding:
      glob: cwl
  - id: mapfile_deep_high_padded
    type: File
    outputBinding:
      glob: cwl_deep_high_padded_size
  - id: mapfile_deep_high_size
    type: File
    outputBinding:
      glob: cwl_deep_high_size
  - id: mapfile_deep_low_padded_size
    type: File
    outputBinding:
      glob: cwl_deep_low_padded_size
  - id: mapfile_deep_low_size
    type: File
    outputBinding:
      glob: cwl_deep_low_size
  - id: mapfile_freqstep
    type: File
    outputBinding:
      glob: cwl_freqstep
  - id: mapfile_high_padded_size
    type: File
    outputBinding:
      glob: cwl_high_padded_size
  - id: mapfile_high_size
    type: File
    outputBinding:
      glob: cwl_high_size
  - id: mapfile_low_padded_size
    type: File
    outputBinding:
      glob: cwl_low_padded_size
  - id: mapfile_low_size
    type: File
    outputBinding:
      glob: cwl_low_size
  - id: mapfile_nbands
    type: File
    outputBinding:
      glob: cwl_nbands
  - id: mapfile_nchansout_clean1
    type: File
    outputBinding:
      glob: cwl_nchansout_clean1
  - id: mapfile_nwavelengths_high
    type: File
    outputBinding:
      glob: cwl_nwavelengths_high
  - id: mapfile_nwavelengths_low
    type: File
    outputBinding:
      glob: cwl_nwavelengths_low
  - id: mapfile_single
    type: File
    outputBinding:
      glob: cwl_single
  - id: mapfile_timestep
    type: File
    outputBinding:
      glob: cwl_timestep
label: _do_magic
arguments:
  - position: 0
    prefix: '-c'
    valueFrom: |
      import InitSubtract_deep_sort_and_compute as Isub_deep
      import os
      import sys
      inputfiles = sys.argv[1:]
      print(inputfiles)
      Isub_deep.main(inputfiles,
           outmapname="cwl",
           mapfile_dir=os.path.basename("."))
requirements:
  - class: InlineJavascriptRequirement
$schemas:
  - 'https://schema.org/docs/schema_org_rdfa.html'
's:author':
  's:person.url': 'https://orcid.org/0000-0001-5125-9539'
's:license': 'https://mit-license.org/'
