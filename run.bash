#!/bin/bash

cargo run --release > image.ppm && convert image.ppm image.png
