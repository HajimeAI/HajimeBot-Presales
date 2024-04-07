import * as anchor from "@coral-xyz/anchor";
import type { HajimeTicket } from "../target/types/hajime_ticket";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.HajimeTicket as anchor.Program<HajimeTicket>;

describe("Test", () => {});
