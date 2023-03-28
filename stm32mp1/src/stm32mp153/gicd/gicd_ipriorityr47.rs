#[doc = "Register `GICD_IPRIORITYR47` reader"]
pub struct R(crate::R<GICD_IPRIORITYR47_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR47_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR47_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR47_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR47` writer"]
pub struct W(crate::W<GICD_IPRIORITYR47_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR47_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR47_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR47_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY0` reader - PRIORITY0"]
pub type PRIORITY0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY0` writer - PRIORITY0"]
pub type PRIORITY0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR47_SPEC, u8, u8, 5, O>;
#[doc = "Field `PRIORITY1` reader - PRIORITY1"]
pub type PRIORITY1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY1` writer - PRIORITY1"]
pub type PRIORITY1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR47_SPEC, u8, u8, 5, O>;
#[doc = "Field `PRIORITY2` reader - PRIORITY2"]
pub type PRIORITY2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY2` writer - PRIORITY2"]
pub type PRIORITY2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR47_SPEC, u8, u8, 5, O>;
#[doc = "Field `PRIORITY3` reader - PRIORITY3"]
pub type PRIORITY3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY3` writer - PRIORITY3"]
pub type PRIORITY3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR47_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    pub fn priority0(&self) -> PRIORITY0_R {
        PRIORITY0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    pub fn priority1(&self) -> PRIORITY1_R {
        PRIORITY1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    pub fn priority2(&self) -> PRIORITY2_R {
        PRIORITY2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    pub fn priority3(&self) -> PRIORITY3_R {
        PRIORITY3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    #[must_use]
    pub fn priority0(&mut self) -> PRIORITY0_W<3> {
        PRIORITY0_W::new(self)
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    #[must_use]
    pub fn priority1(&mut self) -> PRIORITY1_W<11> {
        PRIORITY1_W::new(self)
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    #[must_use]
    pub fn priority2(&mut self) -> PRIORITY2_W<19> {
        PRIORITY2_W::new(self)
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    #[must_use]
    pub fn priority3(&mut self) -> PRIORITY3_W<27> {
        PRIORITY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD interrupt priority register 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr47](index.html) module"]
pub struct GICD_IPRIORITYR47_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr47::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR47_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr47::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR47_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR47 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR47_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
