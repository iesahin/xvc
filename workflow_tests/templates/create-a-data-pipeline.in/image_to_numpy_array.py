
import os
import cv2
import numpy as np
import argparse

# Create the parser and add argument
parser = argparse.ArgumentParser()
parser.add_argument('--dir', required=True, help='Directory where the images are stored')
args = parser.parse_args()

# Define the directory where the images are stored
image_dir = args.dir

# Create two empty lists to store the images and their classes
images = []
classes = []

# Loop through each image in the directory
for image_name in os.listdir(image_dir):
    # Read the image using cv2.imread()
    image = cv2.imread(os.path.join(image_dir, image_name))

    # Resize the image to 64x64 using cv2.resize()
    image = cv2.resize(image, (64, 64))

    # Extract the class from the file name
    class_ = int(image_name.split('_')[-1].split('.')[0])

    # Append the image and its class to the respective lists
    images.append(image)
    classes.append(class_)

# Convert the lists of images and classes to numpy arrays using numpy.array()
images = np.array(images)
classes = np.array(classes)

np.save(f'{image_dir}/images.npy', images)
np.save(f'{image_dir}/classes.npy', classes)
