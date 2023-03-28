#[doc = "Register `CR5` reader"]
pub struct R(crate::R<CR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR5` writer"]
pub struct W(crate::W<CR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR5_SPEC>;
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
impl From<crate::W<CR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDVOS` reader - Step Down converter voltage output scaling"]
pub type SDVOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDVOS` writer - Step Down converter voltage output scaling"]
pub type SDVOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR5_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDSC` reader - Step Down converter supplt startup current selection"]
pub type SDSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDSC` writer - Step Down converter supplt startup current selection"]
pub type SDSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR5_SPEC, u8, u8, 3, O>;
#[doc = "Field `BORHC` reader - BORH configuration selection"]
pub type BORHC_R = crate::BitReader<bool>;
#[doc = "Field `BORHC` writer - BORH configuration selection"]
pub type BORHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, bool, O>;
#[doc = "Field `SMPSCFG` reader - VOS configuration selection (non user)"]
pub type SMPSCFG_R = crate::BitReader<bool>;
#[doc = "Field `SMPSCFG` writer - VOS configuration selection (non user)"]
pub type SMPSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, bool, O>;
#[doc = "Field `SDBEN` reader - Enable Step Down converter Bypass mode enabled"]
pub type SDBEN_R = crate::BitReader<bool>;
#[doc = "Field `SDBEN` writer - Enable Step Down converter Bypass mode enabled"]
pub type SDBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, bool, O>;
#[doc = "Field `SDEB` reader - Enable Step Down converter SMPS mode enabled"]
pub type SDEB_R = crate::BitReader<bool>;
#[doc = "Field `SDEB` writer - Enable Step Down converter SMPS mode enabled"]
pub type SDEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    pub fn sdvos(&self) -> SDVOS_R {
        SDVOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    pub fn sdsc(&self) -> SDSC_R {
        SDSC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    pub fn borhc(&self) -> BORHC_R {
        BORHC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    pub fn smpscfg(&self) -> SMPSCFG_R {
        SMPSCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    pub fn sdben(&self) -> SDBEN_R {
        SDBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn sdeb(&self) -> SDEB_R {
        SDEB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    #[must_use]
    pub fn sdvos(&mut self) -> SDVOS_W<0> {
        SDVOS_W::new(self)
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdsc(&mut self) -> SDSC_W<4> {
        SDSC_W::new(self)
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    #[must_use]
    pub fn borhc(&mut self) -> BORHC_W<8> {
        BORHC_W::new(self)
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    #[must_use]
    pub fn smpscfg(&mut self) -> SMPSCFG_W<9> {
        SMPSCFG_W::new(self)
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sdben(&mut self) -> SDBEN_W<14> {
        SDBEN_W::new(self)
    }
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sdeb(&mut self) -> SDEB_W<15> {
        SDEB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr5](index.html) module"]
pub struct CR5_SPEC;
impl crate::RegisterSpec for CR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr5::R](R) reader structure"]
impl crate::Readable for CR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr5::W](W) writer structure"]
impl crate::Writable for CR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR5 to value 0x4270"]
impl crate::Resettable for CR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x4270;
}
