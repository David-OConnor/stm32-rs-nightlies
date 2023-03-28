#[doc = "Register `TIM1_DTR2` reader"]
pub struct R(crate::R<TIM1_DTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_DTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_DTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_DTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_DTR2` writer"]
pub struct W(crate::W<TIM1_DTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_DTR2_SPEC>;
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
impl From<crate::W<TIM1_DTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_DTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGF` reader - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x t<sub>dtg</sub> with t<sub>dtg</sub>=t<sub>DTS</sub>. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=2xt<sub>DTS</sub>. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=8xt<sub>DTS</sub>. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=16xt<sub>DTS</sub>. Example if T<sub>DTS</sub>=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTGF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTGF` writer - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x t<sub>dtg</sub> with t<sub>dtg</sub>=t<sub>DTS</sub>. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=2xt<sub>DTS</sub>. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=8xt<sub>DTS</sub>. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=16xt<sub>DTS</sub>. Example if T<sub>DTS</sub>=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTGF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_DTR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTAE` reader - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTAE_R = crate::BitReader<bool>;
#[doc = "Field `DTAE` writer - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_DTR2_SPEC, bool, O>;
#[doc = "Field `DTPE` reader - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTPE_R = crate::BitReader<bool>;
#[doc = "Field `DTPE` writer - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_DTR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x t<sub>dtg</sub> with t<sub>dtg</sub>=t<sub>DTS</sub>. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=2xt<sub>DTS</sub>. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=8xt<sub>DTS</sub>. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=16xt<sub>DTS</sub>. Example if T<sub>DTS</sub>=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx => DTF=DTGF\\[7:0\\]x t<sub>dtg</sub> with t<sub>dtg</sub>=t<sub>DTS</sub>. DTGF\\[7:5\\]=10x => DTF=(64+DTGF\\[5:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=2xt<sub>DTS</sub>. DTGF\\[7:5\\]=110 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=8xt<sub>DTS</sub>. DTGF\\[7:5\\]=111 => DTF=(32+DTGF\\[4:0\\])xt<sub>dtg</sub> with T<sub>dtg</sub>=16xt<sub>DTS</sub>. Example if T<sub>DTS</sub>=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtgf(&mut self) -> DTGF_W<0> {
        DTGF_W::new(self)
    }
    #[doc = "Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtae(&mut self) -> DTAE_W<16> {
        DTAE_W::new(self)
    }
    #[doc = "Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtpe(&mut self) -> DTPE_W<17> {
        DTPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 timer deadtime register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dtr2](index.html) module"]
pub struct TIM1_DTR2_SPEC;
impl crate::RegisterSpec for TIM1_DTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_dtr2::R](R) reader structure"]
impl crate::Readable for TIM1_DTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_dtr2::W](W) writer structure"]
impl crate::Writable for TIM1_DTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1_DTR2 to value 0"]
impl crate::Resettable for TIM1_DTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
