#[doc = "Register `OTG_GOTGINT` reader"]
pub struct R(crate::R<OTG_GOTGINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GOTGINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GOTGINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GOTGINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GOTGINT` writer"]
pub struct W(crate::W<OTG_GOTGINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GOTGINT_SPEC>;
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
impl From<crate::W<OTG_GOTGINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GOTGINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEDET` reader - SEDET"]
pub type SEDET_R = crate::BitReader<bool>;
#[doc = "Field `SEDET` writer - SEDET"]
pub type SEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
#[doc = "Field `SRSSCHG` reader - SRSSCHG"]
pub type SRSSCHG_R = crate::BitReader<bool>;
#[doc = "Field `SRSSCHG` writer - SRSSCHG"]
pub type SRSSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
#[doc = "Field `HNSSCHG` reader - HNSSCHG"]
pub type HNSSCHG_R = crate::BitReader<bool>;
#[doc = "Field `HNSSCHG` writer - HNSSCHG"]
pub type HNSSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
#[doc = "Field `HNGDET` reader - HNGDET"]
pub type HNGDET_R = crate::BitReader<bool>;
#[doc = "Field `HNGDET` writer - HNGDET"]
pub type HNGDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
#[doc = "Field `ADTOCHG` reader - ADTOCHG"]
pub type ADTOCHG_R = crate::BitReader<bool>;
#[doc = "Field `ADTOCHG` writer - ADTOCHG"]
pub type ADTOCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
#[doc = "Field `DBCDNE` reader - DBCDNE"]
pub type DBCDNE_R = crate::BitReader<bool>;
#[doc = "Field `DBCDNE` writer - DBCDNE"]
pub type DBCDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
#[doc = "Field `IDCHNG` reader - IDCHNG"]
pub type IDCHNG_R = crate::BitReader<bool>;
#[doc = "Field `IDCHNG` writer - IDCHNG"]
pub type IDCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GOTGINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - SEDET"]
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRSSCHG"]
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNSSCHG"]
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - HNGDET"]
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADTOCHG"]
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBCDNE"]
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IDCHNG"]
    #[inline(always)]
    pub fn idchng(&self) -> IDCHNG_R {
        IDCHNG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SEDET"]
    #[inline(always)]
    #[must_use]
    pub fn sedet(&mut self) -> SEDET_W<2> {
        SEDET_W::new(self)
    }
    #[doc = "Bit 8 - SRSSCHG"]
    #[inline(always)]
    #[must_use]
    pub fn srsschg(&mut self) -> SRSSCHG_W<8> {
        SRSSCHG_W::new(self)
    }
    #[doc = "Bit 9 - HNSSCHG"]
    #[inline(always)]
    #[must_use]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<9> {
        HNSSCHG_W::new(self)
    }
    #[doc = "Bit 17 - HNGDET"]
    #[inline(always)]
    #[must_use]
    pub fn hngdet(&mut self) -> HNGDET_W<17> {
        HNGDET_W::new(self)
    }
    #[doc = "Bit 18 - ADTOCHG"]
    #[inline(always)]
    #[must_use]
    pub fn adtochg(&mut self) -> ADTOCHG_W<18> {
        ADTOCHG_W::new(self)
    }
    #[doc = "Bit 19 - DBCDNE"]
    #[inline(always)]
    #[must_use]
    pub fn dbcdne(&mut self) -> DBCDNE_W<19> {
        DBCDNE_W::new(self)
    }
    #[doc = "Bit 20 - IDCHNG"]
    #[inline(always)]
    #[must_use]
    pub fn idchng(&mut self) -> IDCHNG_W<20> {
        IDCHNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gotgint](index.html) module"]
pub struct OTG_GOTGINT_SPEC;
impl crate::RegisterSpec for OTG_GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gotgint::R](R) reader structure"]
impl crate::Readable for OTG_GOTGINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gotgint::W](W) writer structure"]
impl crate::Writable for OTG_GOTGINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_GOTGINT to value 0"]
impl crate::Resettable for OTG_GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
