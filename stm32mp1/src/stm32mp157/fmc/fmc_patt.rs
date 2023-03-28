#[doc = "Register `FMC_PATT` reader"]
pub struct R(crate::R<FMC_PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PATT` writer"]
pub struct W(crate::W<FMC_PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PATT_SPEC>;
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
impl From<crate::W<FMC_PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTSET` reader - ATTSET"]
pub type ATTSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTSET` writer - ATTSET"]
pub type ATTSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTWAIT` reader - ATTWAIT"]
pub type ATTWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTWAIT` writer - ATTWAIT"]
pub type ATTWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHOLD` reader - ATTHOLD"]
pub type ATTHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHOLD` writer - ATTHOLD"]
pub type ATTHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHIZ` reader - ATTHIZ"]
pub type ATTHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHIZ` writer - ATTHIZ"]
pub type ATTHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PATT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ATTSET"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAIT"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLD"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ATTHIZ"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATTSET"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<0> {
        ATTSET_W::new(self)
    }
    #[doc = "Bits 8:15 - ATTWAIT"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<8> {
        ATTWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - ATTHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<16> {
        ATTHOLD_W::new(self)
    }
    #[doc = "Bits 24:31 - ATTHIZ"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<24> {
        ATTHIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_patt](index.html) module"]
pub struct FMC_PATT_SPEC;
impl crate::RegisterSpec for FMC_PATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_patt::R](R) reader structure"]
impl crate::Readable for FMC_PATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_patt::W](W) writer structure"]
impl crate::Writable for FMC_PATT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_PATT to value 0x0a0a_0a0a"]
impl crate::Resettable for FMC_PATT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a0a_0a0a;
}
