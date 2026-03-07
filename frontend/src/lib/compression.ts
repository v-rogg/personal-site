import { decompressSync } from "fflate";

/**
 * Decompress a gzip+base64 encoded string
 */
export function decompress(data: string): string {
	// Decode base64 to Uint8Array
	const binaryString = atob(data);
	const bytes = new Uint8Array(binaryString.length);
	for (let i = 0; i < binaryString.length; i++) {
		bytes[i] = binaryString.charCodeAt(i);
	}

	// Decompress gzip
	const decompressed = decompressSync(bytes);

	// Convert to string
	return new TextDecoder().decode(decompressed);
}
