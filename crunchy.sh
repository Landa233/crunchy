#!/bin/bash

# Request resources:
#SBATCH -c 4
#SBATCH --time=0-3:0:0
#SBATCH --mem=50G
#SBATCH --gres=tmp:1G
#SBATCH --mail-user=jhtb65@durham.ac.uk
#SBATCH --mail-type=ALL
#SBATCH -p test


#Commands to be run:
cargo run --manifest-path z_n_gauge/Cargo.toml --release --example run_sweep configs/phase_diagram.json