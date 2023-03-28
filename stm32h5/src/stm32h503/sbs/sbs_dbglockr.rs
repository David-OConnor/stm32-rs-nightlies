#[doc = "Register `SBS_DBGLOCKR` reader"]
pub struct R(crate::R<SBS_DBGLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_DBGLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_DBGLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_DBGLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_DBGLOCKR` writer"]
pub struct W(crate::W<SBS_DBGLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_DBGLOCKR_SPEC>;
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
impl From<crate::W<SBS_DBGLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_DBGLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGCFG_LOCK` reader - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
pub type DBGCFG_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBGCFG_LOCK` writer - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
pub type DBGCFG_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBS_DBGLOCKR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
    #[inline(always)]
    pub fn dbgcfg_lock(&self) -> DBGCFG_LOCK_R {
        DBGCFG_LOCK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
    #[inline(always)]
    #[must_use]
    pub fn dbgcfg_lock(&mut self) -> DBGCFG_LOCK_W<0> {
        DBGCFG_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS debug lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_dbglockr](index.html) module"]
pub struct SBS_DBGLOCKR_SPEC;
impl crate::RegisterSpec for SBS_DBGLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_dbglockr::R](R) reader structure"]
impl crate::Readable for SBS_DBGLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_dbglockr::W](W) writer structure"]
impl crate::Writable for SBS_DBGLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_DBGLOCKR to value 0xb4"]
impl crate::Resettable for SBS_DBGLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
