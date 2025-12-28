import type { Request, Response } from "express";
import type { Unit09Service } from "./types";
import { requireString } from "../utils/validators";

export interface ModulesControllerDeps {
  unit09Service: Unit09Service;
}

export function createModulesController(deps: ModulesControllerDeps) {
  const { unit09Service } = deps;

  return {
    listModulesByRepo: async (req: Request, res: Response) => {
      const repoKey = requireString(req.params.repoKey, "repoKey");
      const modules = await unit09Service.listModulesByRepo(repoKey);
      res.json({ items: modules });
    }
  };
}
