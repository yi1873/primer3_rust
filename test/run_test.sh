../target/release/primer3_design_byRUST -s test -i select_region.fa -o select_region.primer3Out.byRust.tsv \
    --primerNum 50 --targetLen_min 85 --targetLen_max 150 \
    --dg_Homodimer=-4.5 --dg_Heterodimer=-4.5 --dg_Hairpin=0.1 \
    --primerLen_min 18 --primerLen_max 24 --primerLen_opt 20 \
    --primerTm_min 58 --primerTm_max 61 --primerTm_opt 59 \
    --primerGC_min 40 --primerGC_max 68 \
    --probeLen_min 18 --probeLen_max 24 --probeLen_opt 20 \
    --probeTm_min 60 --probeTm_max 70 --probeTm_opt 63 \
    --probeGC_min 40 --probeGC_max 70
