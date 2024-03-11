import { useQuery } from "react-query";
import apiClient from "./api-common";
import { useState } from "react";

const get_player_info = () => {
    const [token] = useState(localStorage.getItem("token"));
    const [userId] = useState(localStorage.getItem("user"));
    
    return useQuery(
        "player-info",
        async () => {
            const response = await apiClient.get(
                `steam/player-profile`, {
                    headers: {
                        "axum-accountId": userId,
                        Authorization: token,
                    }
                }
            );
            return response.data;
        }
    );
}

export default get_player_info;