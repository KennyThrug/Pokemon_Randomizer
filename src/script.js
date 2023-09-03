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

  function convertToJson(){
    var settings = {
      //Seed
      seed: document.getElementById("Seed").value,
      //Wild Pokemon
      randomize_wild_pokemon: document.getElementById("Randomize Wild Pokemon").checked,
      allow_pokemon_past_generation: document.getElementById("allow_pokemon_future_generation").checked,
      allow_legends_in_wild_pool: false,
      allow_megas_in_wild_pool: false,
      //Trainer Randomization
      randomize_trainer_pokemon: true,
      trainers_scale: true,
      allow_trainer_legendaries: "NoLegends",
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
    //const numberOfCPUs = rust.getFile().create_emerald_rom();
  }
  function createRandomSeed(){
    let chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let str = '';
    for (let i = 0; i < 32; i++){
      str += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    document.getElementById("Seed").value = str;
  }