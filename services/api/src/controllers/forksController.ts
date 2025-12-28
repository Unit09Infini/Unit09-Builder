import type { Request, Response } from "express";
import type { Unit09Service } from "./types";

export interface ForksControllerDeps {
  unit09Service: Unit09Service;
}

export function createForksController(deps: ForksControllerDeps) {
  const { unit09Service } = deps;

  return {
    listForks: async (_req: Request, res: Response) => {
      const forks = await unit09Service.listForks();
      res.json({ items: forks });
    }
  };
}
