#[doc = "Register `HWCFGR6` reader"]
pub struct R(crate::R<HWCFGR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR6` writer"]
pub struct W(crate::W<HWCFGR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR6_SPEC>;
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
impl From<crate::W<HWCFGR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CPUEVENT` writer - HW configuration CPU event generation"]
pub type CPUEVENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cpuevent(&mut self) -> CPUEVENT_W<0> {
        CPUEVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr6](index.html) module"]
pub struct HWCFGR6_SPEC;
impl crate::RegisterSpec for HWCFGR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr6::R](R) reader structure"]
impl crate::Readable for HWCFGR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr6::W](W) writer structure"]
impl crate::Writable for HWCFGR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWCFGR6 to value 0x03"]
impl crate::Resettable for HWCFGR6_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
