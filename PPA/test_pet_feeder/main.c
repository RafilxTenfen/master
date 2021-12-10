#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <unistd.h>
#include <curl/curl.h>

typedef struct Config {
  int id;
  int dispensedTimes;
  int seccondsToDispense;
  int seccondsToDispenseDecrement;
  int gramsAvailable;
  time_t lastTimeDispensed;
  int configuredPortionGrams;
  int sizeGrams;
  char* animal;
} Config;

static Config configs[3] = {
  {
    id: 3,
    dispensedTimes: 0,
    seccondsToDispense: 4,
    seccondsToDispenseDecrement: 4,
    gramsAvailable: 2500,
    lastTimeDispensed: 0,
    configuredPortionGrams: 400,
    sizeGrams: 3000,
    animal: "Dog",
  }, {
    id: 4,
    dispensedTimes: 0,
    seccondsToDispense: 10,
    seccondsToDispenseDecrement: 10,
    gramsAvailable: 1500,
    lastTimeDispensed: 0,
    configuredPortionGrams: 250,
    sizeGrams: 3000,
    animal: "Cat",
  }, {
    id: 5,
    dispensedTimes: 0,
    seccondsToDispense: 15,
    seccondsToDispenseDecrement: 15,
    gramsAvailable: 6000,
    lastTimeDispensed: 0,
    configuredPortionGrams: 800,
    sizeGrams: 10000,
    animal: "Cow",
  }
};



char* getMessageConfig(struct Config config) {
  char *msg = malloc (sizeof (char) * 400);
  sprintf(msg, "%d,%d,%d,%ld,%d,%d,%s", config.id,
    config.dispensedTimes, config.gramsAvailable, config.lastTimeDispensed,
    config.configuredPortionGrams, config.sizeGrams, config.animal);
  return (char *) msg;
};


char* getJsonConfig(struct Config config) {
  char *postmsg = malloc (sizeof (char) * 10000);
  sprintf(postmsg, "{\n \"id\": \"%d\",\n \"dispensedTimes\": %d,\n \"gramsAvailable\": %d,\n \"lastTimeDispensed\": \"%ld\",\n \"configuredPortionGrams\": %d,\n \"sizeGrams\": %d,\n \"animal\": \"%s\"\n}", config.id,
    config.dispensedTimes, config.gramsAvailable, config.lastTimeDispensed,
    config.configuredPortionGrams, config.sizeGrams, config.animal);
  return (char *) postmsg;
};

// void sendCurl(struct Config config) {
//   CURL *curl;
//   CURLcode res;
//   struct curl_slist *slist1;

//   /* In windows, this will init the winsock stuff */
//   curl_global_init(CURL_GLOBAL_ALL);

//   slist1 = NULL;
//   slist1 = curl_slist_append(slist1, "Content-Type: application/json");
//   curl = curl_easy_init();
//   if (curl) {
//     char *postthis = getJsonConfig(config);
//     printf("\n Sending POST: %s \n", postthis);
//     curl_easy_setopt(curl, CURLOPT_URL, "https://animalfeeder-api-2wmdhpbuoq-uc.a.run.app");
//     curl_easy_setopt(curl, CURLOPT_POSTFIELDS, postthis);
//     curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE_LARGE, (curl_off_t)strlen(postthis));
//     curl_easy_setopt(curl, CURLOPT_HTTPHEADER, slist1);
//     curl_easy_setopt(curl, CURLOPT_USERAGENT, "curl/7.74.0");
//     curl_easy_setopt(curl, CURLOPT_HTTP_VERSION, (long)CURL_HTTP_VERSION_2TLS);
//     curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "POST");

//     res = curl_easy_perform(curl);
//     if (res != CURLE_OK) {
//       fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
//     }

//     curl_easy_cleanup(curl);
//     curl_slist_free_all(slist1);
//   }
// }
void sendCurl(struct Config config) {
  char *postthis = getJsonConfig(config);
  char *command = malloc (sizeof (char) * 10000);
  sprintf(command, "curl --header \"Content-Type: application/json\" \
                                                                             --request POST \
                                                                             --data '%s' \
                                                                             https://animalfeeder-api-2wmdhpbuoq-uc.a.run.app", postthis);
  printf("\nLook command: %s", command);
  system(command);
}

void* createConfig() {
  struct Config* configs;
  configs = malloc(sizeof(struct Config) * 3);
  time_t now = time(0);
  struct Config dog = {
    id: 1,
    dispensedTimes: 0,
    seccondsToDispense: 4,
    seccondsToDispenseDecrement: 4,
    gramsAvailable: 2500,
    lastTimeDispensed: now,
    configuredPortionGrams: 400,
    sizeGrams: 3000,
    animal: "Dog",
  };
  configs[0] = dog;

  struct Config cat = {
    id: 2,
    dispensedTimes: 0,
    seccondsToDispense: 10,
    seccondsToDispenseDecrement: 10,
    gramsAvailable: 1500,
    lastTimeDispensed: now,
    configuredPortionGrams: 250,
    sizeGrams: 3000,
    animal: "Cat",
  };
  configs[1] = cat;

  struct Config cow = {
    id: 3,
    dispensedTimes: 0,
    seccondsToDispense: 15,
    seccondsToDispenseDecrement: 15,
    gramsAvailable: 6000,
    lastTimeDispensed: now,
    configuredPortionGrams: 800,
    sizeGrams: 10000,
    animal: "Cow",
  };
  configs[2] = cow;
  return configs;
}

int main(int argc, char **argv) {
  time_t now = time(0);
  printf("Secconds: %ld", now);

  int numberOfConfigs = 3;
  // Config* configs = createConfig();

  while (1) {
    // Config currentConfig = configs[1];
    // char* msg = getMessageConfig(currentConfig);
    // printf("\nMSG conf ai %s", msg);
    // char * idStr = strtok(msg, ",");
    // printf("\nMSG conf ID: %s", idStr);
    // int id = atoi(idStr);
    // if (id == 1) {
    //   printf("\nIT is my ID: %d - %s", id, msg);
    // }


    for (int i = 0; i < numberOfConfigs; i++) {
      Config currentConfig = configs[i];
      if (currentConfig.seccondsToDispenseDecrement == 0) {
        if (currentConfig.gramsAvailable < currentConfig.configuredPortionGrams) {
          printf("\n\nConfig id: %d doesn't have sufficient grams available %d to configured portion %d",
            currentConfig.id, currentConfig.gramsAvailable, currentConfig.configuredPortionGrams);
          currentConfig.seccondsToDispenseDecrement = currentConfig.seccondsToDispense;
          configs[i] = currentConfig;
          continue;
        }
        currentConfig.dispensedTimes += 1;
        currentConfig.seccondsToDispenseDecrement = currentConfig.seccondsToDispense;
        currentConfig.lastTimeDispensed = time(0);
        currentConfig.gramsAvailable -= currentConfig.configuredPortionGrams;
        configs[i] = currentConfig;
        sendCurl(currentConfig);
        continue;
      }
      currentConfig.seccondsToDispenseDecrement -= 1;
      configs[i] = currentConfig;
    }
    printf("\nPassed 1 second");
    sleep(1);

  }




  return 0;
}
