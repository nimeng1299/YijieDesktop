<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    let { player, score, side, countdown } = $props();
    let dateNow = $state(Date.now());
    let countTime = $state(-1);

    // 定义一个计时器，每秒更新一次 dateNow 的值
    let timer;

    onMount(() => {
        timer = setInterval(() => {
            dateNow = Date.now();
            countTime = Math.floor(countdown - dateNow / 1000);
        }, 1000);
    });

    // 组件卸载时清除计时器，防止内存泄漏
    onDestroy(() => {
        clearInterval(timer);
    });
    function getPlayerName(player) {
        if (player === "-1") {
            return "未加入";
        } else {
            return player;
        }
    }

    function getBtnHidden(player) {
        if (player === "-1") {
            return false;
        } else {
            return true;
        }
    }
    function getScoreHidden(score) {
        if (score === -1) {
            return true;
        } else {
            return false;
        }
    }
    function toBePlayer() {
        if (side === "black") {
            invoke("request_be_chess_player", { side: "1" });
        } else {
            invoke("request_be_chess_player", { side: "2" });
        }
    }

    function getCountdownHidden(c) {
        if (c < 0) {
            return true;
        } else {
            return false;
        }
    }
</script>

<div class="card bg-base-100 card-xs shadow-sm">
    <div class="card-body">
        <div class="flex items-center justify-between">
            <h2 class="card-title">{getPlayerName(player)}</h2>
            <button
                class="btn btn-ghost"
                hidden={getBtnHidden(player)}
                onclick={toBePlayer}>入座</button
            >
        </div>
        <p hidden={getScoreHidden(score)}>得分：{score}</p>
        <p hidden={getCountdownHidden(countTime)}>
            倒计时：{countTime}
        </p>
    </div>
</div>
