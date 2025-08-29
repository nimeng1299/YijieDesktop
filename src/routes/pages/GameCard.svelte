<script>
    let {player, score} = $props();

    function getPlayerName(player) {
        if (player === "-1"){
            return "未加入";
        }else{
            return player ;
        }
    }

    function getBtnHidden(player) {
        if (player === "-1"){
            return false;
        }else{
            return true;
        }
    }
    function getScoreHidden(score) {
        if (score === -1){
            return true;
        }else{
            return false;
        }
    }
</script>
<div class="card bg-base-100 card-xs shadow-sm">
  <div class="card-body">
    <div class="flex items-center justify-between">
     <h2 class="card-title">{getPlayerName(player)}</h2>
        <button class="btn btn-ghost" hidden={getBtnHidden(player)}>入座</button>
    </div>
    <p hidden={getScoreHidden(score)}>得分：{score}</p>
  </div>
</div>
