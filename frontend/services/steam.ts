import apiClient from "./api-common";
import { useState } from "react";

export const get_player_info = async () => {
    const [token] = useState(localStorage.getItem("token"));
    const [userId] = useState(localStorage.getItem("user"));

    const response = await apiClient.get(`steam/player-profile`, {
        headers: {
            "axum-accountId": userId,
            Authorization: token,
        },
    });
    return response.data;
};

export default {
    get_player_info,
};
