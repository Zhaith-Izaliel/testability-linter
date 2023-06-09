package arbreLexicographique.v3;

public class Noeud extends NoeudAbstrait {
	private NoeudAbstrait fils;
	private char valeur;

	public Noeud(NoeudAbstrait frere, NoeudAbstrait fils, char valeur) {
		super(frere);
		if (frere == null)
			throw new IllegalArgumentException("Frere null interdit");
		if (fils == null || fils instanceof NoeudVide)
			throw new IllegalArgumentException("Fils requis");
		this.fils = fils;
		this.valeur = valeur;
	}

	@Override
	public boolean contient(String s) {
		if (s.isEmpty()) 
			return false;
		char c = s.charAt(0);
		if (c == valeur)
			return fils.contient(s.substring(1));
		if (c < valeur)
			return false;
		return frere.contient(s);
	}

	@Override
	public boolean prefixe(String s) {
		if (s.isEmpty()) 
			return true;
		char c = s.charAt(0);
		if (c == valeur)
			return fils.prefixe(s.substring(1));
		if (c < valeur)
			return false;
		return frere.prefixe(s);
	}

	@Override
	public int nbMots() {
		return fils.nbMots() + frere.nbMots();
	}

	@Override
	public NoeudAbstrait ajout(String s) throws ModificationImpossibleException {
		if (s.isEmpty()) 
			return new Marque(this);
		char c = s.charAt(0);
		if (c == valeur) {
			fils = fils.ajout(s.substring(1));
			return this;
		}
		if (c < valeur) {
			NoeudAbstrait n = NoeudVide.getInstance().ajout(s);
			n.frere = this;
			return n;
		}
		frere = frere.ajout(s);
		return this;
	}

	@Override
	public NoeudAbstrait suppr(String s)  throws ModificationImpossibleException {
		if (s.isEmpty()) 
			throw new ModificationImpossibleException("suppression impossible");
		char c = s.charAt(0);
		if (c == valeur) {
			fils = fils.suppr(s.substring(1));
			if (fils instanceof NoeudVide)
				return frere;
			return this;
		}
		if (c < valeur) 
			throw new ModificationImpossibleException("suppression impossible");
		frere = frere.suppr(s);
		return this;
	}

	@Override
	public String toString(String prefixe) {
		return fils.toString(prefixe + valeur) + frere.toString(prefixe);
	}

	
	
	
	
	
	
	
	
	
	
	
	
}
