#[doc = "Register `DDRCTRL_ADDRMAP9` reader"]
pub struct R(crate::R<DDRCTRL_ADDRMAP9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ADDRMAP9` writer"]
pub struct W(crate::W<DDRCTRL_ADDRMAP9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP9_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMAP_ROW_B2` reader - ADDRMAP_ROW_B2"]
pub type ADDRMAP_ROW_B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B2` writer - ADDRMAP_ROW_B2"]
pub type ADDRMAP_ROW_B2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP9_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B3` reader - ADDRMAP_ROW_B3"]
pub type ADDRMAP_ROW_B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B3` writer - ADDRMAP_ROW_B3"]
pub type ADDRMAP_ROW_B3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP9_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B4` reader - ADDRMAP_ROW_B4"]
pub type ADDRMAP_ROW_B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B4` writer - ADDRMAP_ROW_B4"]
pub type ADDRMAP_ROW_B4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP9_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B5` reader - ADDRMAP_ROW_B5"]
pub type ADDRMAP_ROW_B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B5` writer - ADDRMAP_ROW_B5"]
pub type ADDRMAP_ROW_B5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP9_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B2"]
    #[inline(always)]
    pub fn addrmap_row_b2(&self) -> ADDRMAP_ROW_B2_R {
        ADDRMAP_ROW_B2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B3"]
    #[inline(always)]
    pub fn addrmap_row_b3(&self) -> ADDRMAP_ROW_B3_R {
        ADDRMAP_ROW_B3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B4"]
    #[inline(always)]
    pub fn addrmap_row_b4(&self) -> ADDRMAP_ROW_B4_R {
        ADDRMAP_ROW_B4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B5"]
    #[inline(always)]
    pub fn addrmap_row_b5(&self) -> ADDRMAP_ROW_B5_R {
        ADDRMAP_ROW_B5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B2"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b2(&mut self) -> ADDRMAP_ROW_B2_W<0> {
        ADDRMAP_ROW_B2_W::new(self)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B3"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b3(&mut self) -> ADDRMAP_ROW_B3_W<8> {
        ADDRMAP_ROW_B3_W::new(self)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B4"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b4(&mut self) -> ADDRMAP_ROW_B4_W<16> {
        ADDRMAP_ROW_B4_W::new(self)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B5"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b5(&mut self) -> ADDRMAP_ROW_B5_W<24> {
        ADDRMAP_ROW_B5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL address map register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap9](index.html) module"]
pub struct DDRCTRL_ADDRMAP9_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_addrmap9::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap9::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP9 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
