# Rust S3 Image Processing Monorepo

This monorepo project demonstrates a complete serverless workflow using Rust for an end-to-end image processing pipeline on AWS.

## Project Overview

The goal of this project is to create a robust system that automatically resizes uploaded PNG images into three different standard sizes (e.g., small, medium, large) and stores the results in a separate S3 bucket.

The workflow is as follows:

1.  A user uploads an original PNG image to the **Source S3 Bucket** via the .
2.  The S3 bucket triggers an AWS Lambda function via an event notification.
3.  The AWS Lambda function, written entirely in **Rust**, downloads the image.
4.  Using the  crate and the  crate, it resizes the original image into three new PNG formats.
5.  The resized images are uploaded to the **Destination S3 Bucket**.

## Monorepo Structure

The project is structured as a Cargo workspace containing several crates and a separate directory for infrastructure management:


