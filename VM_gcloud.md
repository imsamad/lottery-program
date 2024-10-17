```bash


gcloud auth login

gcloud config set project project-id-1q1232
```

### Create VM

```sh

gclound compute instances create vm-unique-name \
--zone=us-central1 \
--machine-type=e2-medium \
--image-family=debian-10 \
--image-project=debian-cloud \
--boot-disk-size=20gb \
--boot-disk-type=pd-standard
```

### Stop VM

```bash
gcloud compute instances stop vm-unique-name --zone=us-central1
```

### Start VM

```bash
gcloud compute instances start vm-unique-name --zone=us-central1
```

### Delete VM

```bash
gcloud compute instances delete vm-unique-name --zone=us-central1
```

### List out All VM

```bash
gcloud compute instances list --zone=us-central1
```

### SSH into VM

```bash
gcloud compute instances ssh vm-unique-name --zone=us-central1
```

### Display details of VM

```bash
gcloud compute instances describe vm-unique-name --zone=us-central1
```

### Attach disk to VM

```bash
gcloud compute instances attach-disk vm-unique-name --zone=us-central1 --disk=disk-name
```

### Deattach disk to VM

```bash
gcloud compute instances detach-disk vm-unique-name --zone=us-central1 --disk=disk-name
```
