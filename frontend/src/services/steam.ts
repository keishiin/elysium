import { useQuery } from "react-query";
import apiClient from "./api-common";
import { useState } from "react";

export const get_player_info = () => {
    const [token] = useState(localStorage.getItem("token"));
    const [userId] = useState(localStorage.getItem("user"));

    return useQuery("player-info", async () => {
        const response = await apiClient.get(`steam/player-profile`, {
            headers: {
                "axum-accountId": userId,
                Authorization: token,
            },
        });
        return response.data;
    });
};

export const get_onwed_games = (cursor: number) => {
    const [token] = useState(localStorage.getItem("token"));
    const [userId] = useState(localStorage.getItem("user"));

    let url: string;

    if (cursor > 0) {
        url = `steam/games?cursor=${cursor}`;
    } else {
        url = `steam/games`;
    }

    return useQuery("owned-games", async () => {
        const response = await apiClient.get(url, {
            headers: {
                "axum-accountId": userId,
                Authorization: token,
            },
        });
        return response.data;
    });
};


export default {
    get_player_info,
    get_onwed_games,
};