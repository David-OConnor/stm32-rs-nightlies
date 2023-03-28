#[doc = "Register `IP_HWCFGR0` reader"]
pub struct R(crate::R<IP_HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IP_HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IP_HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IP_HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IP_HWCFGR0` writer"]
pub struct W(crate::W<IP_HWCFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IP_HWCFGR0_SPEC>;
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
impl From<crate::W<IP_HWCFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IP_HWCFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL` reader - Dual DAC capability"]
pub type DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUAL` writer - Dual DAC capability"]
pub type DUAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IP_HWCFGR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `LFSR` reader - Pseudonoise wave generation capability"]
pub type LFSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFSR` writer - Pseudonoise wave generation capability"]
pub type LFSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IP_HWCFGR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIANGLE` reader - Triangle wave generation capability"]
pub type TRIANGLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIANGLE` writer - Triangle wave generation capability"]
pub type TRIANGLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IP_HWCFGR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAMPLE` reader - Sample and hold mode capability"]
pub type SAMPLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLE` writer - Sample and hold mode capability"]
pub type SAMPLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IP_HWCFGR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `OR_CFG` reader - option register bit width"]
pub type OR_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OR_CFG` writer - option register bit width"]
pub type OR_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IP_HWCFGR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Dual DAC capability"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pseudonoise wave generation capability"]
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Triangle wave generation capability"]
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Sample and hold mode capability"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - option register bit width"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dual DAC capability"]
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<0> {
        DUAL_W::new(self)
    }
    #[doc = "Bits 4:7 - Pseudonoise wave generation capability"]
    #[inline(always)]
    #[must_use]
    pub fn lfsr(&mut self) -> LFSR_W<4> {
        LFSR_W::new(self)
    }
    #[doc = "Bits 8:11 - Triangle wave generation capability"]
    #[inline(always)]
    #[must_use]
    pub fn triangle(&mut self) -> TRIANGLE_W<8> {
        TRIANGLE_W::new(self)
    }
    #[doc = "Bits 12:15 - Sample and hold mode capability"]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SAMPLE_W<12> {
        SAMPLE_W::new(self)
    }
    #[doc = "Bits 16:23 - option register bit width"]
    #[inline(always)]
    #[must_use]
    pub fn or_cfg(&mut self) -> OR_CFG_W<16> {
        OR_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC IP Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip_hwcfgr0](index.html) module"]
pub struct IP_HWCFGR0_SPEC;
impl crate::RegisterSpec for IP_HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ip_hwcfgr0::R](R) reader structure"]
impl crate::Readable for IP_HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ip_hwcfgr0::W](W) writer structure"]
impl crate::Writable for IP_HWCFGR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IP_HWCFGR0 to value 0x1111"]
impl crate::Resettable for IP_HWCFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1111;
}
