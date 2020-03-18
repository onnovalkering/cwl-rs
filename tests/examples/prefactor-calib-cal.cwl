# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

cwlVersion: v1.0
class: CommandLineTool
baseCommand: [/usr/bin/calibrate-stand-alone]

label: "BBS calibrate"

hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

requirements:
  - class: EnvVarRequirement
    envDef:
      LOFARROOT: /usr
  - class: InitialWorkDirRequirement
    listing:
      - entry: $(inputs.observation)
        writable: true

inputs:
  observation:
    type: Directory
    inputBinding:
      position: 1

  catalog:
    type: File
    inputBinding:
      position: 3

arguments:
 - valueFrom: $(runtime.cores)
   prefix: --numthreads

 - valueFrom: /usr/share/prefactor/parsets/calibcal.parset
   position: 2

outputs:
  mscalib:
    type: Directory
    outputBinding:
      glob: $(inputs.observation.basename)


$namespaces:
  s: http://schema.org/
$schemas:
- https://schema.org/docs/schema_org_rdfa.html


s:license: "https://mit-license.org/"
s:author:
  s:person.url: "http://orcid.org/0000-0002-6136-3724"
