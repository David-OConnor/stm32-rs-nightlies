#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator. Refer to for additional information."]
pub type PLL1FRACEN_R = crate::BitReader<PLL1FRACEN_A>;
#[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator. Refer to for additional information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1FRACEN_A {
    #[doc = "0: Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    Reset = 0,
    #[doc = "1: Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    Set = 1,
}
impl From<PLL1FRACEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1FRACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL1FRACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1FRACEN_A {
        match self.bits {
            false => PLL1FRACEN_A::Reset,
            true => PLL1FRACEN_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PLL1FRACEN_A::Reset
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PLL1FRACEN_A::Set
    }
}
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator. Refer to for additional information."]
pub type PLL1FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLL1FRACEN_A, O>;
impl<'a, const O: u8> PLL1FRACEN_W<'a, O> {
    #[doc = "Reset latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PLL1FRACEN_A::Reset)
    }
    #[doc = "Set latch to tranfer FRACN to the Sigma-Delta modulator"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PLL1FRACEN_A::Set)
    }
}
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. These bits must be written before enabling the PLL1."]
pub type PLL1VCOSEL_R = crate::BitReader<PLL1VCOSEL_A>;
#[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. These bits must be written before enabling the PLL1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1VCOSEL_A {
    #[doc = "0: VCO frequency range 192 to 836 MHz"]
    WideVco = 0,
    #[doc = "1: VCO frequency range 150 to 420 MHz"]
    MediumVco = 1,
}
impl From<PLL1VCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1VCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL1VCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1VCOSEL_A {
        match self.bits {
            false => PLL1VCOSEL_A::WideVco,
            true => PLL1VCOSEL_A::MediumVco,
        }
    }
    #[doc = "Checks if the value of the field is `WideVco`"]
    #[inline(always)]
    pub fn is_wide_vco(&self) -> bool {
        *self == PLL1VCOSEL_A::WideVco
    }
    #[doc = "Checks if the value of the field is `MediumVco`"]
    #[inline(always)]
    pub fn is_medium_vco(&self) -> bool {
        *self == PLL1VCOSEL_A::MediumVco
    }
}
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. These bits must be written before enabling the PLL1."]
pub type PLL1VCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, PLL1VCOSEL_A, O>;
impl<'a, const O: u8> PLL1VCOSEL_W<'a, O> {
    #[doc = "VCO frequency range 192 to 836 MHz"]
    #[inline(always)]
    pub fn wide_vco(self) -> &'a mut W {
        self.variant(PLL1VCOSEL_A::WideVco)
    }
    #[doc = "VCO frequency range 150 to 420 MHz"]
    #[inline(always)]
    pub fn medium_vco(self) -> &'a mut W {
        self.variant(PLL1VCOSEL_A::MediumVco)
    }
}
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_R = crate::FieldReader<u8, PLL1RGE_A>;
#[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1RGE_A {
    #[doc = "0: Frequency is between 1 and 2 MHz"]
    Range1 = 0,
    #[doc = "1: Frequency is between 2 and 4 MHz"]
    Range2 = 1,
    #[doc = "2: Frequency is between 4 and 8 MHz"]
    Range4 = 2,
    #[doc = "3: Frequency is between 8 and 16 MHz"]
    Range8 = 3,
}
impl From<PLL1RGE_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1RGE_A) -> Self {
        variant as _
    }
}
impl PLL1RGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1RGE_A {
        match self.bits {
            0 => PLL1RGE_A::Range1,
            1 => PLL1RGE_A::Range2,
            2 => PLL1RGE_A::Range4,
            3 => PLL1RGE_A::Range8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Range1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == PLL1RGE_A::Range1
    }
    #[doc = "Checks if the value of the field is `Range2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == PLL1RGE_A::Range2
    }
    #[doc = "Checks if the value of the field is `Range4`"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == PLL1RGE_A::Range4
    }
    #[doc = "Checks if the value of the field is `Range8`"]
    #[inline(always)]
    pub fn is_range8(&self) -> bool {
        *self == PLL1RGE_A::Range8
    }
}
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLCFGR_SPEC, u8, PLL1RGE_A, 2, O>;
impl<'a, const O: u8> PLL1RGE_W<'a, O> {
    #[doc = "Frequency is between 1 and 2 MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(PLL1RGE_A::Range1)
    }
    #[doc = "Frequency is between 2 and 4 MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(PLL1RGE_A::Range2)
    }
    #[doc = "Frequency is between 4 and 8 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(PLL1RGE_A::Range4)
    }
    #[doc = "Frequency is between 8 and 16 MHz"]
    #[inline(always)]
    pub fn range8(self) -> &'a mut W {
        self.variant(PLL1RGE_A::Range8)
    }
}
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN2 into the sigma-delta modulator. In order to latch the FRACN2 value into the sigma-delta modulator, PLL2FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN2 into the modulator. Refer to for additional information."]
pub use PLL1FRACEN_R as PLL2FRACEN_R;
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator. Refer to for additional information."]
pub use PLL1FRACEN_R as PLL3FRACEN_R;
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN2 into the sigma-delta modulator. In order to latch the FRACN2 value into the sigma-delta modulator, PLL2FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN2 into the modulator. Refer to for additional information."]
pub use PLL1FRACEN_W as PLL2FRACEN_W;
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator. Refer to for additional information."]
pub use PLL1FRACEN_W as PLL3FRACEN_W;
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
pub use PLL1RGE_R as PLL2RGE_R;
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3."]
pub use PLL1RGE_R as PLL3RGE_R;
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
pub use PLL1RGE_W as PLL2RGE_W;
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3."]
pub use PLL1RGE_W as PLL3RGE_W;
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
pub use PLL1VCOSEL_R as PLL2VCOSEL_R;
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub use PLL1VCOSEL_R as PLL3VCOSEL_R;
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
pub use PLL1VCOSEL_W as PLL2VCOSEL_W;
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
pub use PLL1VCOSEL_W as PLL3VCOSEL_W;
#[doc = "Field `DIVP1EN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type DIVP1EN_R = crate::BitReader<DIVP1EN_A>;
#[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVP1EN_A {
    #[doc = "0: Clock ouput is disabled"]
    Disabled = 0,
    #[doc = "1: Clock output is enabled"]
    Enabled = 1,
}
impl From<DIVP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIVP1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DIVP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVP1EN_A {
        match self.bits {
            false => DIVP1EN_A::Disabled,
            true => DIVP1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIVP1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIVP1EN_A::Enabled
    }
}
#[doc = "Field `DIVP1EN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type DIVP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, DIVP1EN_A, O>;
impl<'a, const O: u8> DIVP1EN_W<'a, O> {
    #[doc = "Clock ouput is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIVP1EN_A::Disabled)
    }
    #[doc = "Clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIVP1EN_A::Enabled)
    }
}
#[doc = "Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use DIVP1EN_R as DIVQ1EN_R;
#[doc = "Field `DIVR1EN` reader - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use DIVP1EN_R as DIVR1EN_R;
#[doc = "Field `DIVP2EN` reader - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
pub use DIVP1EN_R as DIVP2EN_R;
#[doc = "Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
pub use DIVP1EN_R as DIVQ2EN_R;
#[doc = "Field `DIVR2EN` reader - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
pub use DIVP1EN_R as DIVR2EN_R;
#[doc = "Field `DIVP3EN` reader - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
pub use DIVP1EN_R as DIVP3EN_R;
#[doc = "Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
pub use DIVP1EN_R as DIVQ3EN_R;
#[doc = "Field `DIVR3EN` reader - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
pub use DIVP1EN_R as DIVR3EN_R;
#[doc = "Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use DIVP1EN_W as DIVQ1EN_W;
#[doc = "Field `DIVR1EN` writer - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub use DIVP1EN_W as DIVR1EN_W;
#[doc = "Field `DIVP2EN` writer - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
pub use DIVP1EN_W as DIVP2EN_W;
#[doc = "Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
pub use DIVP1EN_W as DIVQ2EN_W;
#[doc = "Field `DIVR2EN` writer - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
pub use DIVP1EN_W as DIVR2EN_W;
#[doc = "Field `DIVP3EN` writer - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
pub use DIVP1EN_W as DIVP3EN_W;
#[doc = "Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
pub use DIVP1EN_W as DIVQ3EN_W;
#[doc = "Field `DIVR3EN` writer - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
pub use DIVP1EN_W as DIVR3EN_W;
impl R {
    #[doc = "Bit 0 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator. Refer to for additional information."]
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. These bits must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN2 into the sigma-delta modulator. In order to latch the FRACN2 value into the sigma-delta modulator, PLL2FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN2 into the modulator. Refer to for additional information."]
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn pll2vcosel(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator. Refer to for additional information."]
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> PLL3VCOSEL_R {
        PLL3VCOSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3."]
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    pub fn divp1en(&self) -> DIVP1EN_R {
        DIVP1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn divq1en(&self) -> DIVQ1EN_R {
        DIVQ1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn divr1en(&self) -> DIVR1EN_R {
        DIVR1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    pub fn divp2en(&self) -> DIVP2EN_R {
        DIVP2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
    #[inline(always)]
    pub fn divq2en(&self) -> DIVQ2EN_R {
        DIVQ2EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
    #[inline(always)]
    pub fn divr2en(&self) -> DIVR2EN_R {
        DIVR2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    pub fn divp3en(&self) -> DIVP3EN_R {
        DIVP3EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
    #[inline(always)]
    pub fn divq3en(&self) -> DIVQ3EN_R {
        DIVQ3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
    #[inline(always)]
    pub fn divr3en(&self) -> DIVR3EN_R {
        DIVR3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<0> {
        PLL1FRACEN_W::new(self)
    }
    #[doc = "Bit 1 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. These bits must be written before enabling the PLL1."]
    #[inline(always)]
    #[must_use]
    pub fn pll1vcosel(&mut self) -> PLL1VCOSEL_W<1> {
        PLL1VCOSEL_W::new(self)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<2> {
        PLL1RGE_W::new(self)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable Set and reset by software to latch the content of FRACN2 into the sigma-delta modulator. In order to latch the FRACN2 value into the sigma-delta modulator, PLL2FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN2 into the modulator. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<4> {
        PLL2FRACEN_W::new(self)
    }
    #[doc = "Bit 5 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
    #[inline(always)]
    #[must_use]
    pub fn pll2vcosel(&mut self) -> PLL2VCOSEL_W<5> {
        PLL2VCOSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
    #[inline(always)]
    #[must_use]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<6> {
        PLL2RGE_W::new(self)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable Set and reset by software to latch the content of FRACN3 into the sigma-delta modulator. In order to latch the FRACN3 value into the sigma-delta modulator, PLL3FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN3 into the modulator. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<8> {
        PLL3FRACEN_W::new(self)
    }
    #[doc = "Bit 9 - PLL3 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL3. This bit must be written before enabling the PLL3."]
    #[inline(always)]
    #[must_use]
    pub fn pll3vcosel(&mut self) -> PLL3VCOSEL_W<9> {
        PLL3VCOSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. These bits must be written before enabling the PLL3."]
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<10> {
        PLL3RGE_W::new(self)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn divp1en(&mut self) -> DIVP1EN_W<16> {
        DIVP1EN_W::new(self)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn divq1en(&mut self) -> DIVQ1EN_W<17> {
        DIVQ1EN_W::new(self)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn divr1en(&mut self) -> DIVR1EN_W<18> {
        DIVR1EN_W::new(self)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    #[must_use]
    pub fn divp2en(&mut self) -> DIVP2EN_W<19> {
        DIVP2EN_W::new(self)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn divq2en(&mut self) -> DIVQ2EN_W<20> {
        DIVQ2EN_W::new(self)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn divr2en(&mut self) -> DIVR2EN_W<21> {
        DIVR2EN_W::new(self)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used."]
    #[inline(always)]
    #[must_use]
    pub fn divp3en(&mut self) -> DIVP3EN_W<22> {
        DIVP3EN_W::new(self)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn divq3en(&mut self) -> DIVQ3EN_W<23> {
        DIVQ3EN_W::new(self)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, DIVR3EN and DIVR3 bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn divr3en(&mut self) -> DIVR3EN_W<24> {
        DIVR3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x01ff_0000"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_0000;
}
