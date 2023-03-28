#[doc = "Register `DDRPHYC_DXCCR` reader"]
pub struct R(crate::R<DDRPHYC_DXCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DXCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DXCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DXCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DXCCR` writer"]
pub struct W(crate::W<DDRPHYC_DXCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DXCCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DXCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DXCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DXODT` reader - DXODT"]
pub type DXODT_R = crate::BitReader<bool>;
#[doc = "Field `DXODT` writer - DXODT"]
pub type DXODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
#[doc = "Field `DXIOM` reader - DXIOM"]
pub type DXIOM_R = crate::BitReader<bool>;
#[doc = "Field `DXIOM` writer - DXIOM"]
pub type DXIOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
#[doc = "Field `DXPDD` reader - DXPDD"]
pub type DXPDD_R = crate::BitReader<bool>;
#[doc = "Field `DXPDD` writer - DXPDD"]
pub type DXPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
#[doc = "Field `DXPDR` reader - DXPDR"]
pub type DXPDR_R = crate::BitReader<bool>;
#[doc = "Field `DXPDR` writer - DXPDR"]
pub type DXPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
#[doc = "Field `DQSRES` reader - DQSRES"]
pub type DQSRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSRES` writer - DQSRES"]
pub type DQSRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DXCCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DQSNRES` reader - DQSNRES"]
pub type DQSNRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSNRES` writer - DQSNRES"]
pub type DQSNRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DXCCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DQSNRST` reader - DQSNRST"]
pub type DQSNRST_R = crate::BitReader<bool>;
#[doc = "Field `DQSNRST` writer - DQSNRST"]
pub type DQSNRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
#[doc = "Field `RVSEL` reader - RVSEL"]
pub type RVSEL_R = crate::BitReader<bool>;
#[doc = "Field `RVSEL` writer - RVSEL"]
pub type RVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
#[doc = "Field `AWDT` reader - AWDT"]
pub type AWDT_R = crate::BitReader<bool>;
#[doc = "Field `AWDT` writer - AWDT"]
pub type AWDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DXCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    pub fn dxodt(&self) -> DXODT_R {
        DXODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    pub fn dqsres(&self) -> DQSRES_R {
        DQSRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    pub fn dqsnres(&self) -> DQSNRES_R {
        DQSNRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    pub fn dqsnrst(&self) -> DQSNRST_R {
        DQSNRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    pub fn rvsel(&self) -> RVSEL_R {
        RVSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    pub fn awdt(&self) -> AWDT_R {
        AWDT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    #[must_use]
    pub fn dxodt(&mut self) -> DXODT_W<0> {
        DXODT_W::new(self)
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    #[must_use]
    pub fn dxiom(&mut self) -> DXIOM_W<1> {
        DXIOM_W::new(self)
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    #[must_use]
    pub fn dxpdd(&mut self) -> DXPDD_W<2> {
        DXPDD_W::new(self)
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    #[must_use]
    pub fn dxpdr(&mut self) -> DXPDR_W<3> {
        DXPDR_W::new(self)
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    #[must_use]
    pub fn dqsres(&mut self) -> DQSRES_W<4> {
        DQSRES_W::new(self)
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    #[must_use]
    pub fn dqsnres(&mut self) -> DQSNRES_W<8> {
        DQSNRES_W::new(self)
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    #[must_use]
    pub fn dqsnrst(&mut self) -> DQSNRST_W<14> {
        DQSNRST_W::new(self)
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rvsel(&mut self) -> RVSEL_W<15> {
        RVSEL_W::new(self)
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    #[must_use]
    pub fn awdt(&mut self) -> AWDT_W<16> {
        AWDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DXCC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dxccr](index.html) module"]
pub struct DDRPHYC_DXCCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DXCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dxccr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DXCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dxccr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DXCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DXCCR to value 0x0800"]
impl crate::Resettable for DDRPHYC_DXCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
