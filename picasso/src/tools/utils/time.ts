

export function getDayDifference(date1: Date, date2: Date): number {
    const msInDay = 1000 * 60 * 60 * 24;
    return Math.round((date2.getTime() - date1.getTime()) / msInDay);
}
