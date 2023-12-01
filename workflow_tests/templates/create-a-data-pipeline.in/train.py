import argparse
import json
import torch
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import numpy as np
import yaml
from sklearn.metrics import confusion_matrix


# Define the CNN model
class Net(nn.Module):
    def __init__(self):
        super(Net, self).__init__()
        self.conv1 = nn.Conv2d(3, 6, 5)
        self.pool = nn.MaxPool2d(2, 2)
        self.conv2 = nn.Conv2d(6, 16, 5)
        self.fc1 = nn.Linear(16 * 13 * 13, 120)
        self.fc2 = nn.Linear(120, 84)
        self.fc3 = nn.Linear(84, 15)

    def forward(self, x):
        x = self.pool(F.relu(self.conv1(x)))
        x = self.pool(F.relu(self.conv2(x)))
        x = x.view(-1, 16 * 13 * 13)
        x = F.relu(self.fc1(x))
        x = F.relu(self.fc2(x))
        x = self.fc3(x)
        return x


# Parse command line arguments
parser = argparse.ArgumentParser()
parser.add_argument("--train_dir", type=str, required=True)
parser.add_argument("--val_dir", type=str, required=True)
parser.add_argument("--test_dir", type=str, required=True)
args = parser.parse_args()

# Load hyperparameters from yaml file
with open("params.yaml") as file:
    params = yaml.safe_load(file)
batch_size = params["batch_size"]
epochs = params["epochs"]

# Initialize the model, loss function and optimizer
model = Net()
criterion = nn.CrossEntropyLoss()
optimizer = optim.SGD(model.parameters(), lr=0.001, momentum=0.9)

# Load the training dataset
train_images = np.load(args.train_dir + "/images.npy")
train_labels = np.load(args.train_dir + "/classes.npy")
train_images = torch.from_numpy(train_images).float()
train_labels = torch.from_numpy(train_labels).long()
train_dataset = TensorDataset(train_images, train_labels)
trainloader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)

# Load the validation dataset
val_images = np.load(args.val_dir + "/images.npy")
val_labels = np.load(args.val_dir + "/classes.npy")
val_images = torch.from_numpy(val_images).float()
val_labels = torch.from_numpy(val_labels).long()
val_dataset = TensorDataset(val_images, val_labels)
valloader = DataLoader(val_dataset, batch_size=batch_size, shuffle=True)

# Train the model
for epoch in range(epochs):  # loop over the dataset multiple times
    running_loss = 0.0
    for i, data in enumerate(trainloader, 0):
        inputs, labels = data
        print(inputs.shape)
        inputs = inputs.permute(0, 3, 1, 2)
        optimizer.zero_grad()
        outputs = model(inputs)
        loss = criterion(outputs, labels)
        loss.backward()
        optimizer.step()

        running_loss += loss.item()
        if i % 2000 == 1999:  # print every 2000 mini-batches
            print("[%d, %5d] loss: %.3f" % (epoch + 1, i + 1, running_loss / 2000))
            running_loss = 0.0

    # Validate the model
    correct = 0
    total = 0
    with torch.no_grad():
        for data in valloader:
            images, labels = data
            outputs = model(images)
            _, predicted = torch.max(outputs.data, 1)
            total += labels.size(0)
            correct += (predicted == labels).sum().item()

    print(
        "Accuracy of the network on the validation images: %d %%"
        % (100 * correct / total)
    )

# Save the trained model
torch.save(model.state_dict(), "model.pth")

# Load the test dataset
test_images = np.load(args.test_dir + "/images.npy")
test_labels = np.load(args.test_dir + "/classes.npy")
test_images = torch.from_numpy(test_images).float()
test_labels = torch.from_numpy(test_labels).long()
test_dataset = TensorDataset(test_images, test_labels)
testloader = DataLoader(test_dataset, batch_size=batch_size, shuffle=True)

# Test the model
correct = 0
total = 0
all_labels = []
all_predictions = []
with torch.no_grad():
    for data in testloader:
        images, labels = data
        outputs = model(images)
        _, predicted = torch.max(outputs.data, 1)
        total += labels.size(0)
        correct += (predicted == labels).sum().item()
        all_labels.extend(labels)
        all_predictions.extend(predicted)

# Compute confusion matrix
conf_mat = confusion_matrix(all_labels, all_predictions)
print("Confusion Matrix:")
print(conf_mat)

# Save confusion matrix to a json file
with open("results.json", "w") as f:
    json.dump(conf_mat.tolist(), f)
