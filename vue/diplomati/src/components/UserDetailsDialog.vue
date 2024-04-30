<template>
  <v-card>
    <v-card-title>
      {{ userTitle }}
    </v-card-title>
    <v-list>
      <v-list-item v-if="userType === 'D'">
        <v-list-item-title>Numero REA: {{ user.numero_rea }}</v-list-item-title>
        <v-list-item-title>Provincia: {{ user.provincia }}</v-list-item-title>
        <v-list-item-title>Codice Fiscale: {{ user.codice_fiscale }}</v-list-item-title>
        <v-list-item-title>Forma Giuridica: {{ user.forma_giuridica }}</v-list-item-title>
        <v-list-item-title>Descrizione Attività: {{ user.descrizione_attivita }}</v-list-item-title>
      </v-list-item>
      <v-list-item v-if="userType === 'A'">
        <v-list-item-title>Indirizzo Studio: {{ user.indirizzo_studio }}</v-list-item-title>
        <v-list-item-title>Voto Maturità: {{ user.voto_maturita }}</v-list-item-title>
        <v-list-item-title>Certificazioni Acquisite: {{ user.certificazioni_acquisite }}</v-list-item-title>
        <v-list-item-title>Esperienze Lavorative: {{ user.esperienze_lavorative }}</v-list-item-title>
      </v-list-item>
    </v-list>
    <v-card-actions v-if="userType === 'D'">
      <v-btn color="primary" @click="$emit('addCV')">Aggiungi CV</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script>
export default {
  props: ['user', 'userType'],
  computed: {
    userTitle() {
      return this.user.denominazione_azienda || this.user.nome;
    }
  }
};
</script>