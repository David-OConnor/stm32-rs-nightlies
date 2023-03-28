#[doc = "Register `MEMRMP` reader"]
pub struct R(crate::R<MEMRMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMRMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMRMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMRMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMRMP` writer"]
pub struct W(crate::W<MEMRMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMRMP_SPEC>;
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
impl From<crate::W<MEMRMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMRMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_BOOT` reader - Memory mapping selection"]
pub type MEM_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `MEM_BOOT` writer - Memory mapping selection"]
pub type MEM_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMRMP_SPEC, bool, O>;
#[doc = "Field `SWP_FMC` reader - FMC memory mapping swap"]
pub type SWP_FMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWP_FMC` writer - FMC memory mapping swap"]
pub type SWP_FMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMRMP_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_boot(&self) -> MEM_BOOT_R {
        MEM_BOOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_boot(&mut self) -> MEM_BOOT_W<0> {
        MEM_BOOT_W::new(self)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    #[must_use]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W<10> {
        SWP_FMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memrmp](index.html) module"]
pub struct MEMRMP_SPEC;
impl crate::RegisterSpec for MEMRMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memrmp::R](R) reader structure"]
impl crate::Readable for MEMRMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memrmp::W](W) writer structure"]
impl crate::Writable for MEMRMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MEMRMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
