# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

cwlVersion: v1.0
class: CommandLineTool
baseCommand: [python, /usr/lib/prefactor/scripts/fit_clocktec_initialguess_losoto.py]

label: "Separate clock/TEC"

hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

inputs:
  globaldbname:
    type: File
    format: http://revoltek.github.io/losoto/cookbook.pdf
    doc: "input h5 parm file"
    inputBinding:
      position: 1

arguments:
 # calsource
 - valueFrom: "fitclock"
   position: 2

 # ncpus
 - valueFrom: $(runtime.cores)
   position: 3

outputs:
  dclock_1st:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Clock solutions"
    outputBinding:
      glob: fitted_data_dclock_fitclock_1st.npy

  dTEC_1st:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "TEC solutions"
    outputBinding:
      glob: fitted_data_dTEC_fitclock_1st.npy

  dclock_1st_sm:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Smoothed clock solutions"
    outputBinding:
      glob: fitted_data_dclock_fitclock_1st.sm.npy

  dTEC_1st_sm:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Smoothed TEC solutions"
    outputBinding:
      glob: fitted_data_dTEC_fitclock_1st.sm.npy



$namespaces:
  s: http://schema.org/
$schemas:
  - https://schema.org/version/latest/schema.rdf

s:license: "https://mit-license.org/"
s:author:
  s:person.url: "http://orcid.org/0000-0002-6136-3724"
