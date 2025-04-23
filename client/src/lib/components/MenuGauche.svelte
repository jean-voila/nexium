<script>
    import Bouton from '$lib/components/Bouton.svelte';
    import { SendHorizontal } from 'lucide-svelte';
    import { HandCoins } from 'lucide-svelte';
    import { QrCode } from 'lucide-svelte';
    import { Share2 } from 'lucide-svelte';
    import { Copy } from 'lucide-svelte';
</script>


<div class="flex flex-col gap-14 justify-center ">

    <div class="flex flex-col gap-2">
        <div class="flex flex-col">
            <div class="prenom">Valentina</div>
            <div class="nom">Duretz</div>
        </div>
        <div class="flex gap-1">
            <button>
                <QrCode strokeWidth={2.4} size={25} class="bouton-action" />
            </button>
            
            <button>
                <Share2 strokeWidth={2.4} size={25} class="bouton-action"/>
            </button>

            <button>
                <Copy strokeWidth={2.4} size={25} class="bouton-action"/>
            </button>
        </div>
    </div>

    <div class="flex flex-col gap-2">
        <Bouton label="Transférer" Icon={SendHorizontal}/>
        <Bouton label="Facturer" type="secondaire" Icon={HandCoins}/>
    </div>

</div>