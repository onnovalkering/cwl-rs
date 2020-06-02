# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

cwlVersion: v1.0
class: CommandLineTool
baseCommand: [python]

label: "Plot clock, TEC, amplitude"

hints:
  DockerRequirement:
      dockerPull: kernsuite/prefactor

inputs:
  amplitude_array:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html

  dclock_1st_sm:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html

  dtec_1st_sm:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html

outputs:
  dtec_allsols:
    type: File
    format: image/png
    outputBinding:
      glob: dtec_allsols.png

  dclock_allsols:
    type: File
    format: image/png
    outputBinding:
      glob: dclock_allsols.png

  amp_allsols:
    type: File
    format: image/png
    outputBinding:
      glob: amp_allsols.png

arguments:
  - prefix: -c
    valueFrom: |
      import matplotlib as mpl
      mpl.use("Agg")
      import numpy as np
      import pylab

      amparray   = np.load("$(inputs.amplitude_array.path)")
      clockarray = np.load("$(inputs.dclock_1st_sm.path)")
      dtecarray  = np.load("$(inputs.dtec_1st_sm.path)")
      numants = len(dtecarray[0,:])

      for i in range(0,numants):
          pylab.plot(dtecarray[:,i])
      pylab.xlabel("Time")
      pylab.ylabel("dTEC [$10^{16}$ m$^{-2}$]")
      pylab.savefig("dtec_allsols.png")
      pylab.close()
      pylab.cla()

      for i in range(0,numants):
          pylab.plot(1e9*clockarray[:,i])
      pylab.xlabel("Time")
      pylab.ylabel("dClock [ns]")
      pylab.savefig("dclock_allsols.png")
      pylab.close()
      pylab.cla()


      for i in range(0,numants):
        pylab.plot(np.median(amparray[i,:,:,0], axis=0))
        pylab.plot(np.median(amparray[i,:,:,1], axis=0))
      pylab.xlabel("Subband number")
      pylab.ylabel("Amplitude")
      pylab.ylim(0,2.*np.median(amparray))
      pylab.savefig("amp_allsols.png")
      pylab.close()
      pylab.cla()


$namespaces:
  s: http://schema.org/
$schemas:
  - https://schema.org/version/latest/schema.rdf



s:license: "https://mit-license.org/"
s:author:
  s:person.url: "http://orcid.org/0000-0002-6136-3724"
