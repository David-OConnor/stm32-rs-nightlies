#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXMSGDISCCF` writer - Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register."]
pub type TXMSGDISCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXMSGSENTCF` writer - Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the UCPD_SR register."]
pub type TXMSGSENTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXMSGABTCF` writer - Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the UCPD_SR register."]
pub type TXMSGABTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `HRSTDISCCF` writer - Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the UCPD_SR register."]
pub type HRSTDISCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `HRSTSENTCF` writer - Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the UCPD_SR register."]
pub type HRSTSENTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXUNDCF` writer - Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the UCPD_SR register."]
pub type TXUNDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXORDDETCF` writer - Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the UCPD_SR register."]
pub type RXORDDETCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXHRSTDETCF` writer - Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the UCPD_SR register."]
pub type RXHRSTDETCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXOVRCF` writer - Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the UCPD_SR register."]
pub type RXOVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXMSGENDCF` writer - Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the UCPD_SR register."]
pub type RXMSGENDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TYPECEVT1CF` writer - Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register"]
pub type TYPECEVT1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TYPECEVT2CF` writer - Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register"]
pub type TYPECEVT2CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FRSEVTCF` writer - FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the UCPD_SR register."]
pub type FRSEVTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<1> {
        TXMSGDISCCF_W::new(self)
    }
    #[doc = "Bit 2 - Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<2> {
        TXMSGSENTCF_W::new(self)
    }
    #[doc = "Bit 3 - Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<3> {
        TXMSGABTCF_W::new(self)
    }
    #[doc = "Bit 4 - Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<4> {
        HRSTDISCCF_W::new(self)
    }
    #[doc = "Bit 5 - Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<5> {
        HRSTSENTCF_W::new(self)
    }
    #[doc = "Bit 6 - Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn txundcf(&mut self) -> TXUNDCF_W<6> {
        TXUNDCF_W::new(self)
    }
    #[doc = "Bit 9 - Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<9> {
        RXORDDETCF_W::new(self)
    }
    #[doc = "Bit 10 - Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<10> {
        RXHRSTDETCF_W::new(self)
    }
    #[doc = "Bit 11 - Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<11> {
        RXOVRCF_W::new(self)
    }
    #[doc = "Bit 12 - Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<12> {
        RXMSGENDCF_W::new(self)
    }
    #[doc = "Bit 14 - Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<14> {
        TYPECEVT1CF_W::new(self)
    }
    #[doc = "Bit 15 - Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<15> {
        TYPECEVT2CF_W::new(self)
    }
    #[doc = "Bit 20 - FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the UCPD_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<20> {
        FRSEVTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
