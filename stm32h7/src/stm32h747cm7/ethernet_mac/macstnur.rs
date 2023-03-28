#[doc = "Register `MACSTNUR` reader"]
pub struct R(crate::R<MACSTNUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTNUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTNUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTNUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSTNUR` writer"]
pub struct W(crate::W<MACSTNUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSTNUR_SPEC>;
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
impl From<crate::W<MACSTNUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSTNUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSS` reader - Timestamp Sub-seconds"]
pub type TSSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub-seconds"]
pub type TSSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACSTNUR_SPEC, u32, u32, 31, O>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time"]
pub type ADDSUB_R = crate::BitReader<bool>;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time"]
pub type ADDSUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSTNUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<0> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<31> {
        ADDSUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System time nanoseconds update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstnur](index.html) module"]
pub struct MACSTNUR_SPEC;
impl crate::RegisterSpec for MACSTNUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macstnur::R](R) reader structure"]
impl crate::Readable for MACSTNUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macstnur::W](W) writer structure"]
impl crate::Writable for MACSTNUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSTNUR to value 0"]
impl crate::Resettable for MACSTNUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
