#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC0` reader - PRESC0"]
pub type PRESC0_R = crate::BitReader<bool>;
#[doc = "Field `PRESC0` writer - PRESC0"]
pub type PRESC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `PRESC1` reader - PRESC1"]
pub type PRESC1_R = crate::BitReader<bool>;
#[doc = "Field `PRESC1` writer - PRESC1"]
pub type PRESC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `PRESC2` reader - PRESC2"]
pub type PRESC2_R = crate::BitReader<bool>;
#[doc = "Field `PRESC2` writer - PRESC2"]
pub type PRESC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `PRESC3` reader - PRESC3"]
pub type PRESC3_R = crate::BitReader<bool>;
#[doc = "Field `PRESC3` writer - PRESC3"]
pub type PRESC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `TSEN` reader - TSEN"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - TSEN"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `VBATEN` reader - VBATEN"]
pub type VBATEN_R = crate::BitReader<bool>;
#[doc = "Field `VBATEN` writer - VBATEN"]
pub type VBATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 18 - PRESC0"]
    #[inline(always)]
    pub fn presc0(&self) -> PRESC0_R {
        PRESC0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PRESC1"]
    #[inline(always)]
    pub fn presc1(&self) -> PRESC1_R {
        PRESC1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PRESC2"]
    #[inline(always)]
    pub fn presc2(&self) -> PRESC2_R {
        PRESC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PRESC3"]
    #[inline(always)]
    pub fn presc3(&self) -> PRESC3_R {
        PRESC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - PRESC0"]
    #[inline(always)]
    #[must_use]
    pub fn presc0(&mut self) -> PRESC0_W<18> {
        PRESC0_W::new(self)
    }
    #[doc = "Bit 19 - PRESC1"]
    #[inline(always)]
    #[must_use]
    pub fn presc1(&mut self) -> PRESC1_W<19> {
        PRESC1_W::new(self)
    }
    #[doc = "Bit 20 - PRESC2"]
    #[inline(always)]
    #[must_use]
    pub fn presc2(&mut self) -> PRESC2_W<20> {
        PRESC2_W::new(self)
    }
    #[doc = "Bit 21 - PRESC3"]
    #[inline(always)]
    #[must_use]
    pub fn presc3(&mut self) -> PRESC3_W<21> {
        PRESC3_W::new(self)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<23> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<24> {
        VBATEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
