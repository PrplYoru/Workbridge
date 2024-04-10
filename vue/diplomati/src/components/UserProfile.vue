<script setup>

</script>

<template>
  <v-container>
    <v-row>
      <v-col cols="12" md="6">
        <v-card>
          <v-card-title>Dettagli utente</v-card-title>
          <v-card-text>
            <v-text-field label="Specializzazione" v-model="details.specializzazione" required></v-text-field>
            <v-text-field label="Indirizzo studio" v-model="details.indirizzo_studio" required></v-text-field>
            <v-text-field label="Voto maturità" v-model="details.voto_maturita" required></v-text-field>
            <v-text-field label="Certificazioni acquisite" v-model="details.certificazioni_acquisite" required></v-text-field>
            <v-text-field label="Esperienze lavorative" v-model="details.esperienze_lavorative" required></v-text-field>
            <v-text-field label="Denominazione azienda" v-model="details.denominazione_azienda" required></v-text-field>
            <v-text-field label="Numero REA" v-model="details.numero_rea" required></v-text-field>
            <v-text-field label="Codice fiscale" v-model="details.codice_fiscale" required></v-text-field>
            <v-text-field label="Forma giuridica" v-model="details.forma_giuridica" required></v-text-field>
            <v-text-field label="Descrizione attività" v-model="details.descrizione_attivita" required></v-text-field>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="6">
        <v-card>
          <v-card-title>Dettagli azienda</v-card-title>
          <v-card-text>
            <v-text-field label="Categoria" v-model="details.categoria" required></v-text-field>
            <v-text-field label="Indirizzo" v-model="details.indirizzo" required></v-text-field>
            <v-text-field label="Contatti" v-model="details.contatti" required></v-text-field>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>

</style>