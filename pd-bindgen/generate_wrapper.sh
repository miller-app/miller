#!/bin/sh

# Generates wrapper.h
# Usage:
#   generate_wrapper <git-tag to checkout pd at>

tag=$1

update_pd() {
    rm -rf pure-data
    git clone https://github.com/pure-data/pure-data.git pure-data
    cd pure-data || exit
    git checkout tags/"${tag}"
    cd .. || exit
}

rm_unused() {
    cd pure-data || exit

    unused_stuff=".git \
                 .gitattributes \
                 .travis.yml \
                 asio \
                 doc \
                 font \
                 linux  \
                 m4 \
                 mac  \
                 man \
                 md \
                 msw  \
                 po \
                 portaudio \
                 portmidi  \
                 src/d_fft_fftw.c \
                 src/s_audio_alsa.c \
                 src/s_audio_alsa.h \
                 src/s_audio_alsamm.c \
                 src/s_audio_audiounit.c \
                 src/s_audio_esd.c \
                 src/s_audio_jack.c \
                 src/s_audio_mmio.c \
                 src/s_audio_oss.c \
                 src/s_audio_pa.c \
                 src/s_audio_paring.c \
                 src/s_audio_paring.h \
                 src/s_entry.c \
                 src/s_midi.c \
                 src/s_midi_alsa.c \
                 src/s_midi_dummy.c \
                 src/s_midi_mmio.c \
                 src/s_midi_oss.c \
                 src/s_midi_pm.c \
                 src/s_watchdog.c \
                 src/u_pdreceive.c \
                 src/u_pdsend.c \
                 tcl"

    for un in ${unused_stuff}; do
        rm -rf "${un}"
    done

    cd - || exit
}

generate_wrapper() {
    rm -f wrapper.h
    touch wrapper.h

    cd pure-data/src || exit

    echo "#include \"m_pd.h\"" >> ../../wrapper.h

    mv m_pd.h ../

    for h in *.h; do
        echo "#include \"${h}\"" >> ../../wrapper.h
    done

    mv ../m_pd.h .

    cd - || exit
}

update_pd
rm_unused
generate_wrapper
