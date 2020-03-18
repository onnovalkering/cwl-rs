# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

class: CommandLineTool
cwlVersion: v1.0
$namespaces:
  s: 'http://schema.org/'
id: create_image

hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

baseCommand: [python]
inputs:
  - id: mapfile_deep_low_size
    type: File
  - id: mapfile_nchansout_clean1
    type: File
  - id: mapfile_nwavelengths_low
    type: File
  - id: image_padding
    type: float
  - id: maxlambda_lowres
    type: int
  - id: cellsize_lowres_deg
    type: float
  - id: prepcals
    type: 'Directory[]'
    inputBinding:
      position: 19
outputs:
  - id: psf
    type: File
    outputBinding:
      glob: wsclean-MFS-psf.fits
  - id: dirty
    type: File
    outputBinding:
      glob: wsclean-MFS-dirty.fits
  - id: residual
    type: File
    outputBinding:
      glob: wsclean-MFS-residual.fits
  - id: model
    type: File
    outputBinding:
      glob: wsclean-MFS-model.fits
label: create_image
arguments:
  - prefix: '-c'
    valueFrom: |
      import ast
      import sys
      import subprocess as SP
      cmlopts = ["wsclean"]
      padfil = open("$(inputs.mapfile_deep_low_size.path)")
      cmlopts.append("-size")
      cmlopts.extend(ast.literal_eval(padfil.read())[0]["file"].split())
      ncfil = open("$(inputs.mapfile_nchansout_clean1.path)")
      cmlopts.append("-channels-out")
      cmlopts.append(ast.literal_eval(ncfil.read())[0]["file"])
      nwfil = open("$(inputs.mapfile_nwavelengths_low.path)")
      cmlopts.append("-baseline-averaging")
      cmlopts.append(ast.literal_eval(nwfil.read())[0]["file"])
      cmlopts.extend(sys.argv[1:])
      print " ".join(cmlopts)
      SP.call(cmlopts)
  - prefix: '-padding'
    valueFrom: $(inputs.image_padding)
  - prefix: '-niter'
    valueFrom: '20000'
  - prefix: '-threshold'
    valueFrom: '0.0'
  - prefix: '-pol'
    valueFrom: I
  - prefix: '-weight'
    valueFrom: briggs
  - valueFrom: '0.0'
  - prefix: '-mgain'
    valueFrom: '0.65'
  - prefix: '-minuv-l'
    valueFrom: '80'
  - prefix: '-maxuv-l'
    valueFrom: $(inputs.maxlambda_lowres)
  - prefix: '-weighting-rank-filter'
    valueFrom: '3'
  - prefix: '-temp-dir'
    valueFrom: $(runtime.tmpdir)
  - prefix: '-scale'
    valueFrom: $(inputs.cellsize_lowres_deg)
  - prefix: '-j'
    valueFrom: $(runtime.cores)
  - prefix: '-abs-mem'
    valueFrom: $(runtime.ram)
  - -no-update-model-required
  - -reorder
  - -fit-beam
  - -join-channels
requirements:
  - class: InlineJavascriptRequirement
$schemas:
  - 'https://schema.org/docs/schema_org_rdfa.html'
's:author':
  's:person.url': 'https://orcid.org/0000-0001-5125-9539'
's:license': 'https://mit-license.org/'
