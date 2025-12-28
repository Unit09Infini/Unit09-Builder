import type { Request, Response } from "express";
import type { Unit09Service } from "./types";

export interface StatsControllerDeps {
  unit09Service: Unit09Service;
}

export function createStatsController(deps: StatsControllerDeps) {
  const { unit09Service } = deps;

  return {
    getStats: async (_req: Request, res: Response) => {
      const stats = await unit09Service.getGlobalStats();
      res.json({ stats });
    }
  };
}
