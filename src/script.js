function openTab(evt, tabName) {
    // Declare all variables
    var i, tabcontent, tablinks;
  
    // Get all elements with class="tabcontent" and hide them
    tabcontent = document.getElementsByClassName("tabcontent");
    for (i = 0; i < tabcontent.length; i++) {
      tabcontent[i].style.display = "none";
    }
  
    // Get all elements with class="tablinks" and remove the class "active"
    tablinks = document.getElementsByClassName("tablinks");
    for (i = 0; i < tablinks.length; i++) {
      tablinks[i].className = tablinks[i].className.replace(" active", "");
    }
  
    // Show the current tab, and add an "active" class to the button that opened the tab
    document.getElementById(tabName).style.display = "block";
    evt.currentTarget.className += " active";
  }

  //Makes only relevant Options Appear in Wild Pokemon Tab
  function updateWildTab(evt){
    if(document.getElementById("Randomize Wild Pokemon").checked){
      document.getElementById("WildPokemonVisible").style.display = "block"
    }
    else{
      document.getElementById("WildPokemonVisible").style.display = "none"
    }
  }

  //Makes only relevant Options Appear in Trainer Tab
  function updateTrainerTab(evt){
    if(document.getElementById("RandomizeEnemyTrainers").checked){
      document.getElementById("TrainersAppear").style.display = "block"
    }
    else{
      document.getElementById("TrainersAppear").style.display = "none"
    }
    if(document.getElementById("TrainersNoLegendary").checked){
      document.getElementById("TrainerLegendaryPokemonRareAppear").style.display = "none";
    }
    else{
      document.getElementById("TrainerLegendaryPokemonRareAppear").style.display = "block";
    }
  }


  //--------------------------------------------All Below this Line is functionallity and Helpers for Generating Seeds -------------------------------------------------------
  function convertToJson(){
    var settings = {
      //Seed
      seed: document.getElementById("Seed").value,
      //Wild Pokemon
      randomize_wild_pokemon: document.getElementById("Randomize Wild Pokemon").checked,
      randomize_starter_pokemon: document.getElementById("randomizeStarter").checked,
      allow_starter_legendary: get_starter_legend(),
      scale_starter: document.getElementById("ScaleStarter").checked,
      allow_pokemon_future_generation: document.getElementById("allow_pokemon_future_generation").checked,
      scale_wild_pokemon: document.getElementById("ScaleWithRoutes").checked,
      allow_legends_in_wild_pool: get_wild_legend(),
      allow_megas_in_wild_pool: get_wild_mega(),
      //Trainer Randomization
      randomize_trainer_pokemon: document.getElementById("RandomizeEnemyTrainers").checked,
      trainers_scale: document.getElementById("TrainerScaleRoutes").checked,
      allow_trainer_legendaries: get_trainer_legends(),
      trainer_legendaries_rare: document.getElementById("TrainerLegendRare").checked,
      //Gym Leader Randomization
      allow_leader_legendaries: "OneLegend",
      gym_type: "RandomType",
      recieve_pokemon_reward_gym: true,
      randomize_gym_locations: false,
      //Item Randomization
      add_rare_candy: 10,
      add_held_items: true,
      add_held_items_later_gens: true,
      items_from_trainers: true,
      important_items_only_from_trainers: false,
      add_pokeballs: 10,
      allow_pokeballs_from_store: false,
      make_balls_reusable: true,
      allow_healing_items: true,
      randomize_hidden_items: false,
      gym_leader_keys: true,
      //Evolution Settings

      //Other Settings
      allow_hm_use: true
    }
    //document.getElementById("H").innerText = settings.randomize_wild_pokemon;
    var settingsJson = JSON.stringify(settings);
    rust.getFile().emerald_rom(settingsJson);
  }
  function get_trainer_legends(){
    if(document.getElementById("TrainerAllowLegends").checked){
      return "AllowLegends";
    }
    if(document.getElementById("TrainersOneLegendary").checked){
      return "OneLegend";
    }
    return "NoLegends";
  }
  function get_wild_legend(){
    if(document.getElementById("AllowLegends").checked){
      return "AllowLegends";
    }
    if(document.getElementById("SometimesLegends").checked){
      return "SometimesLegends";
    }
    return "NoLegends";
  }
  function get_wild_mega(){
    if(document.getElementById("AllowMegas").checked){
      return "AllowLegends";
    }
    if(document.getElementById("SometimesMegas").checked){
      return "SometimesLegends";
    }
    return "NoLegends";
  }
  function get_starter_legend(){
    if(document.getElementById("AllowStarterLegends").checked){
      return "AllowLegends"
    }
    if(document.getElementById("SometimesStarterLegends").checked){
      return "SometimesLegends"
    }
    return "NoLegends"
  }
  function createRandomSeed(){
    let chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let str = '';
    for (let i = 0; i < 32; i++){
      str += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    document.getElementById("Seed").value = str;
  }