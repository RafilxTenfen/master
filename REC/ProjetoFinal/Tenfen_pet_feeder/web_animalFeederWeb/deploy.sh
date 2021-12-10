gcloud builds submit --tag gcr.io/animalfeeder-81959/animalfeeder --project=animalfeeder-81959

gcloud beta run deploy animalfeeder-api --image gcr.io/animalfeeder-81959/animalfeeder --platform managed --region us-central1 --project=animalfeeder-81959