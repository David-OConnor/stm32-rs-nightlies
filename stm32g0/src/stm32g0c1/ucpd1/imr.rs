#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXISIE` reader - TXIS interrupt enable"]
pub type TXISIE_R = crate::BitReader<bool>;
#[doc = "Field `TXISIE` writer - TXIS interrupt enable"]
pub type TXISIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TXMSGDISCIE` reader - TXMSGDISC interrupt enable"]
pub type TXMSGDISCIE_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGDISCIE` writer - TXMSGDISC interrupt enable"]
pub type TXMSGDISCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TXMSGSENTIE` reader - TXMSGSENT interrupt enable"]
pub type TXMSGSENTIE_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGSENTIE` writer - TXMSGSENT interrupt enable"]
pub type TXMSGSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TXMSGABTIE` reader - TXMSGABT interrupt enable"]
pub type TXMSGABTIE_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGABTIE` writer - TXMSGABT interrupt enable"]
pub type TXMSGABTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `HRSTDISCIE` reader - HRSTDISC interrupt enable"]
pub type HRSTDISCIE_R = crate::BitReader<bool>;
#[doc = "Field `HRSTDISCIE` writer - HRSTDISC interrupt enable"]
pub type HRSTDISCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `HRSTSENTIE` reader - HRSTSENT interrupt enable"]
pub type HRSTSENTIE_R = crate::BitReader<bool>;
#[doc = "Field `HRSTSENTIE` writer - HRSTSENT interrupt enable"]
pub type HRSTSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TXUNDIE` reader - TXUND interrupt enable"]
pub type TXUNDIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDIE` writer - TXUND interrupt enable"]
pub type TXUNDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXORDDETIE` reader - RXORDDET interrupt enable"]
pub type RXORDDETIE_R = crate::BitReader<bool>;
#[doc = "Field `RXORDDETIE` writer - RXORDDET interrupt enable"]
pub type RXORDDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXHRSTDETIE` reader - RXHRSTDET interrupt enable"]
pub type RXHRSTDETIE_R = crate::BitReader<bool>;
#[doc = "Field `RXHRSTDETIE` writer - RXHRSTDET interrupt enable"]
pub type RXHRSTDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXOVRIE` reader - RXOVR interrupt enable"]
pub type RXOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVRIE` writer - RXOVR interrupt enable"]
pub type RXOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXMSGENDIE` reader - RXMSGEND interrupt enable"]
pub type RXMSGENDIE_R = crate::BitReader<bool>;
#[doc = "Field `RXMSGENDIE` writer - RXMSGEND interrupt enable"]
pub type RXMSGENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TYPECEVT1IE` reader - TYPECEVT1 interrupt enable"]
pub type TYPECEVT1IE_R = crate::BitReader<bool>;
#[doc = "Field `TYPECEVT1IE` writer - TYPECEVT1 interrupt enable"]
pub type TYPECEVT1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TYPECEVT2IE` reader - TYPECEVT2 interrupt enable"]
pub type TYPECEVT2IE_R = crate::BitReader<bool>;
#[doc = "Field `TYPECEVT2IE` writer - TYPECEVT2 interrupt enable"]
pub type TYPECEVT2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `FRSEVTIE` reader - FRSEVT interrupt enable"]
pub type FRSEVTIE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TXIS interrupt enable"]
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISC interrupt enable"]
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENT interrupt enable"]
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABT interrupt enable"]
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISC interrupt enable"]
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENT interrupt enable"]
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUND interrupt enable"]
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDET interrupt enable"]
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDET interrupt enable"]
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVR interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGEND interrupt enable"]
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1 interrupt enable"]
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2 interrupt enable"]
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - FRSEVT interrupt enable"]
    #[inline(always)]
    pub fn frsevtie(&self) -> FRSEVTIE_R {
        FRSEVTIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXIS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txisie(&mut self) -> TXISIE_W<0> {
        TXISIE_W::new(self)
    }
    #[doc = "Bit 1 - TXMSGDISC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W<1> {
        TXMSGDISCIE_W::new(self)
    }
    #[doc = "Bit 2 - TXMSGSENT interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W<2> {
        TXMSGSENTIE_W::new(self)
    }
    #[doc = "Bit 3 - TXMSGABT interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W<3> {
        TXMSGABTIE_W::new(self)
    }
    #[doc = "Bit 4 - HRSTDISC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W<4> {
        HRSTDISCIE_W::new(self)
    }
    #[doc = "Bit 5 - HRSTSENT interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W<5> {
        HRSTSENTIE_W::new(self)
    }
    #[doc = "Bit 6 - TXUND interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txundie(&mut self) -> TXUNDIE_W<6> {
        TXUNDIE_W::new(self)
    }
    #[doc = "Bit 8 - RXNE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<8> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 9 - RXORDDET interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W<9> {
        RXORDDETIE_W::new(self)
    }
    #[doc = "Bit 10 - RXHRSTDET interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W<10> {
        RXHRSTDETIE_W::new(self)
    }
    #[doc = "Bit 11 - RXOVR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<11> {
        RXOVRIE_W::new(self)
    }
    #[doc = "Bit 12 - RXMSGEND interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W<12> {
        RXMSGENDIE_W::new(self)
    }
    #[doc = "Bit 14 - TYPECEVT1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W<14> {
        TYPECEVT1IE_W::new(self)
    }
    #[doc = "Bit 15 - TYPECEVT2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W<15> {
        TYPECEVT2IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
