export const timeConversion = (minutes: number) => {
    const hours = minutes / 60;
    return Number(hours.toFixed(1));
};

export default timeConversion;
