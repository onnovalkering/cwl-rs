# https://github.com/EOSC-LOFAR/prefactor-cwl (MIT license)

cwlVersion: v1.0
class: Workflow

requirements:
  ScatterFeatureRequirement: {}

inputs:
  ms_array: Directory[]
  reference_station: string
  avg.freqstep: int
  avg.timestep: int
  flag.baseline: string
  maxlambda_lowres: int
  cellsize_lowres_deg: float
  image_padding: float

outputs:
  losoto_h5:
    type: File
    format: http://revoltek.github.io/losoto/cookbook.pdf
    label: "H5Parm solutions"
    outputSource: h5imp_cal/losoto_h5

  dTEC_1st:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "TEC solutions"
    outputSource: fitclock/dTEC_1st

  dTEC_1st_sm:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Smoothed TEC solutions"
    outputSource: fitclock/dTEC_1st_sm

  dclock_1st:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Clock solutions"
    outputSource: fitclock/dclock_1st

  dclock_1st_sm:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Smoothed clock solutions"
    outputSource: fitclock/dclock_1st_sm

  amplitude_array:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Amplitude solutions"
    outputSource: ampl/amplitude_array

  dtec_allsols:
    type: File
    format: iana:image/png
    outputSource: plots/dtec_allsols

  dclock_allsols:
    type: File
    format: iana:image/png
    outputSource: plots/dclock_allsols

  amp_allsols:
    type: File
    format: iana:image/png
    outputSource: plots/amp_allsols

  phase_xx_yy_offset:
    type: File
    format: iana:image/png
    outputSource: phase/phase_xx_yy_offset

  freqs_for_phase_array:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Frequencies"
    outputSource: phase/freqs_for_phase_array

  phase_array:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Phase solutions"
    outputSource: phase/phase_array

  station_names:
    type: File
    format: https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html
    label: "Station names"
    outputSource: phase/station_names

  polXX_dirpointing:
    type: File
    format: iana:image/png
    outputSource: plot_cal_phases/polXX_dirpointing

  polYY_dirpointing:
    type: File
    format: iana:image/png
    outputSource: plot_cal_phases/polYY_dirpointing

  psf:
    type: File
    format: iana:image/fits
    outputSource: create_image/psf

  residual:
    type: File
    format: iana:image/fits
    outputSource: create_image/residual

  model:
    type: File
    format: iana:image/fits
    outputSource: create_image/model

  dirty:
    type: File
    format: iana:image/fits
    outputSource: create_image/dirty


steps:
  ndppp_prep_cal:
    run: steps/ndppp_prep_cal.cwl
    in:
      msin: ms_array
      avg.freqstep: avg.freqstep
      avg.timestep: avg.timestep
      flag.baseline: flag.baseline
    scatter: msin
    out:
        [msout]

  sky_cal:
    run: steps/sky_cal.cwl
    in:
      ms: ndppp_prep_cal/msout
    scatter: ms
    out:
      [skymodel]

  calib_cal:
    run: steps/calib_cal.cwl
    in:
      observation: ndppp_prep_cal/msout
      catalog: sky_cal/skymodel
    scatter:
      - observation
      - catalog
    scatterMethod: dotproduct
    out:
      [mscalib]

  h5imp_cal:
    run: steps/h5imp_cal.cwl
    in:
      ms_array: calib_cal/mscalib
    out:
      [losoto_h5]

  fitclock:
    run: steps/fitclock.cwl
    in:
      globaldbname: h5imp_cal/losoto_h5
    out:
      [dTEC_1st, dTEC_1st_sm, dclock_1st, dclock_1st_sm]

  ampl:
    run: steps/ampl.cwl
    in:
      globaldbname: h5imp_cal/losoto_h5
    out:
      [amplitude_array]

  plots:
    run: steps/plots.cwl
    in:
      amplitude_array: ampl/amplitude_array
      dclock_1st: fitclock/dclock_1st
      dclock_1st_sm: fitclock/dclock_1st_sm
      dtec_1st_sm: fitclock/dTEC_1st_sm
    out:
       [dtec_allsols, dclock_allsols, amp_allsols]

  phase:
    run: steps/phase.cwl
    in:
      losoto: h5imp_cal/losoto_h5
    out:
      - freqs_for_phase_array
      - phase_array
      - station_names
      - phase_xx_yy_offset

  plot_cal_phases:
    run: steps/plot_cal_phases.cwl
    in:
      h5parm: h5imp_cal/losoto_h5
      reference_station: reference_station
    out:
      [polXX_dirpointing, polYY_dirpointing]

  do_magic:
    run: steps/do_magic.cwl
    in:
      prepcals: calib_cal/mscalib
    out:
      - mapfile_paths
      - mapfile_deep_high_padded
      - mapfile_deep_high_size
      - mapfile_deep_low_padded_size
      - mapfile_deep_low_size
      - mapfile_freqstep
      - mapfile_high_padded_size
      - mapfile_high_size
      - mapfile_low_padded_size
      - mapfile_low_size
      - mapfile_nbands
      - mapfile_nchansout_clean1
      - mapfile_nwavelengths_high
      - mapfile_nwavelengths_low
      - mapfile_single
      - mapfile_timestep

  create_image:
    run: steps/create_image.cwl
    in:
      mapfile_deep_low_size: do_magic/mapfile_deep_low_size
      mapfile_nchansout_clean1: do_magic/mapfile_nchansout_clean1
      mapfile_nwavelengths_low: do_magic/mapfile_nwavelengths_low
      prepcals: calib_cal/mscalib
      image_padding: image_padding
      maxlambda_lowres: maxlambda_lowres
      cellsize_lowres_deg: cellsize_lowres_deg
    out:
     - psf
     - dirty
     - residual
     - model

$namespaces:
  s: http://schema.org/
  iana: https://www.iana.org/assignments/media-types/
$schemas:
  - 'https://schema.org/docs/schema_org_rdfa.html'

s:license: 'https://mit-license.org/'
s:author:
  - s:person.url: 'http://orcid.org/0000-0002-6136-3724'
  - s:person.url: 'https://orcid.org/0000-0001-5125-9539'
