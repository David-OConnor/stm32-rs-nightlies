#[doc = "Register `FMAC_PARAM` reader"]
pub struct R(crate::R<FMAC_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMAC_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMAC_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMAC_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMAC_PARAM` writer"]
pub struct W(crate::W<FMAC_PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMAC_PARAM_SPEC>;
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
impl From<crate::W<FMAC_PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMAC_PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - Input parameter P. The value of this parameter is dependent on the function This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P` writer - Input parameter P. The value of this parameter is dependent on the function This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMAC_PARAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `Q` reader - Input parameter Q. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Q` writer - Input parameter Q. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMAC_PARAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `R` reader - Input parameter R. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R` writer - Input parameter R. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMAC_PARAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `FUNC` reader - Function 2: Load X2 buffer 3: Load Y buffer 4 to 7: Reserved 8: Convolution (FIR filter) 9: IIR filter (direct form 1) This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type FUNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC` writer - Function 2: Load X2 buffer 3: Load Y buffer 4 to 7: Reserved 8: Convolution (FIR filter) 9: IIR filter (direct form 1) This bitfield can not be modified when a function is ongoing (START = 1)"]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMAC_PARAM_SPEC, u8, u8, 7, O>;
#[doc = "Field `START` reader - Enable execution Setting this bit triggers the execution of the function selected in the FUNC bitfield. Resetting it by software stops any ongoing function. For initialization functions, this bit is reset by hardware."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Enable execution Setting this bit triggers the execution of the function selected in the FUNC bitfield. Resetting it by software stops any ongoing function. For initialization functions, this bit is reset by hardware."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_PARAM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Input parameter P. The value of this parameter is dependent on the function This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Input parameter Q. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Input parameter R. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Function 2: Load X2 buffer 3: Load Y buffer 4 to 7: Reserved 8: Convolution (FIR filter) 9: IIR filter (direct form 1) This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Enable execution Setting this bit triggers the execution of the function selected in the FUNC bitfield. Resetting it by software stops any ongoing function. For initialization functions, this bit is reset by hardware."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input parameter P. The value of this parameter is dependent on the function This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bits 8:15 - Input parameter Q. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn q(&mut self) -> Q_W<8> {
        Q_W::new(self)
    }
    #[doc = "Bits 16:23 - Input parameter R. The value of this parameter is dependent on the function. This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<16> {
        R_W::new(self)
    }
    #[doc = "Bits 24:30 - Function 2: Load X2 buffer 3: Load Y buffer 4 to 7: Reserved 8: Convolution (FIR filter) 9: IIR filter (direct form 1) This bitfield can not be modified when a function is ongoing (START = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<24> {
        FUNC_W::new(self)
    }
    #[doc = "Bit 31 - Enable execution Setting this bit triggers the execution of the function selected in the FUNC bitfield. Resetting it by software stops any ongoing function. For initialization functions, this bit is reset by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<31> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC parameter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmac_param](index.html) module"]
pub struct FMAC_PARAM_SPEC;
impl crate::RegisterSpec for FMAC_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmac_param::R](R) reader structure"]
impl crate::Readable for FMAC_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmac_param::W](W) writer structure"]
impl crate::Writable for FMAC_PARAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMAC_PARAM to value 0"]
impl crate::Resettable for FMAC_PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
